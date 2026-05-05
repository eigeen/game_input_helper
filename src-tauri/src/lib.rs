use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use tauri::{LogicalPosition, LogicalSize, Manager as _};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;

mod app_ime_state;
mod game_detector;
mod handle;
mod hotkey;
mod ime_guard;
mod input;
mod window_ime;

static PREVENT_NEXT_SHOW: AtomicBool = AtomicBool::new(false);
const APP_IME_STATE_INTERVAL: Duration = Duration::from_millis(500);
const IME_GUARD_INTERVAL: Duration = Duration::from_millis(1000);
const GAME_INPUT_DELAY: Duration = Duration::from_millis(300);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn input(content: String) -> Result<(), ()> {
    tokio::spawn(submit_content_to_game(content));
    Ok(())
}

async fn submit_content_to_game(content: String) {
    let handle = handle::Handle::global();
    let _ = handle.hide_window();

    wait_for_game_focus().await;
    send_content_to_game(&content).await;

    wait_for_game_focus().await;
    submit_game_input().await;

    wait_for_game_focus().await;
    show_input_window(handle);
}

async fn send_content_to_game(content: &str) {
    let input = input::Input::global();
    if let Err(error) = input.input_text_chunked(content).await {
        log::error!("Failed to input text: {error}");
    }
}

async fn submit_game_input() {
    let input = input::Input::global();
    PREVENT_NEXT_SHOW.store(true, Ordering::Relaxed);

    if let Err(error) = input.input_key(enigo::Key::Return).await {
        log::error!("Failed to input return: {error}");
    }
}

async fn wait_for_game_focus() {
    tokio::time::sleep(GAME_INPUT_DELAY).await;
}

fn show_input_window(handle: &handle::Handle) {
    if let Err(error) = handle.show_window() {
        log::error!("Failed to show input window: {error}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // init global tokio runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        input::Input::global()
            .init()
            .await
            .expect("Failed to initialize input");

        let app = tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .plugin(tauri_plugin_global_shortcut::Builder::new().build())
            .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
                let _ = handle::Handle::global().show_window();
            }))
            .setup(|app| {
                let main_window = app.get_webview_window("main").unwrap();
                main_window.set_shadow(true)?;
                main_window.set_size(LogicalSize::new(410.0, 70.0))?;
                main_window.set_position(LogicalPosition::new(1200.0, 800.0))?;
                main_window.set_minimizable(false)?;
                main_window.set_maximizable(false)?;
                main_window.set_title(&format!(
                    "Game Input Helper - v{}",
                    env!("CARGO_PKG_VERSION")
                ))?;

                handle::Handle::init(app.handle());

                // 注册F7强制显示/隐藏热键
                hotkey::Hotkey::global()
                    .register("F7", hotkey::HotkeyFunc::SwitchDisplay)
                    .expect("Failed to register F7 hotkey");

                tokio::spawn(watch_app_ime_state_loop(main_window.clone()));

                // 启动游戏检测线程，使用轮询方式检测按键事件
                // 轮询是由于其他事件驱动的API在游戏环境都不太好使
                // 例如由于引擎Direct Input，系统按键事件无法监听
                // 全局快捷键会阻止按键发送到游戏本身
                tokio::spawn(async {
                    let game_detector = game_detector::GameDetector::new();
                    let ime_guard = ime_guard::ImeGuard::new();
                    let mut last_ime_guard = Instant::now() - IME_GUARD_INTERVAL;

                    loop {
                        guard_game_ime(&game_detector, &ime_guard, &mut last_ime_guard);
                        // 轮询检测Enter键状态
                        let pressed = hotkey::Hotkey::global().is_key_pressed_async(VK_RETURN);

                        // 检测Enter键按下（上升沿）
                        if pressed {
                            // 防止重复显示
                            let is_prevent = PREVENT_NEXT_SHOW.compare_exchange(
                                true,
                                false,
                                Ordering::Relaxed,
                                Ordering::Relaxed,
                            );
                            if is_prevent.is_ok() {
                                continue;
                            }

                            // 游戏在前台
                            if game_detector.is_game_active() {
                                log::info!("Enter key pressed in game, showing window");
                                let handle = handle::Handle::global();
                                let _ = handle.show_window();
                            }
                        }

                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                });

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![input])
            .build(tauri::generate_context!())
            .expect("error while building tauri application");

        app.run(move |_handle, _event| {});
    })
}

fn guard_game_ime(
    game_detector: &game_detector::GameDetector,
    ime_guard: &ime_guard::ImeGuard,
    last_ime_guard: &mut Instant,
) {
    if last_ime_guard.elapsed() < IME_GUARD_INTERVAL {
        return;
    }

    *last_ime_guard = Instant::now();
    if game_detector.is_game_active() {
        let _ = ime_guard.keep_foreground_ime_english();
    }
}

async fn watch_app_ime_state_loop(window: tauri::WebviewWindow) {
    let mut app_ime_state = app_ime_state::AppImeState::new();

    loop {
        if let Err(error) = app_ime_state.update(&window) {
            log::debug!("failed to update app IME state: {error}");
        }

        tokio::time::sleep(APP_IME_STATE_INTERVAL).await;
    }
}
