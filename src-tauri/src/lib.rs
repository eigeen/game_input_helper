use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use tauri::{LogicalPosition, LogicalSize, Manager as _};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;

mod game_detector;
mod handle;
mod hotkey;
mod input;

static PREVENT_NEXT_SHOW: AtomicBool = AtomicBool::new(false);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn input(content: String) -> Result<(), ()> {
    tokio::spawn(async move {
        let handle = handle::Handle::global();
        let _ = handle.hide_window();

        tokio::time::sleep(Duration::from_millis(300)).await;
        let input = input::Input::global();
        if let Err(e) = input.input_text_chunked(&content).await {
            log::error!("Failed to input text: {}", e);
        };

        tokio::time::sleep(Duration::from_millis(300)).await;
        // 防止发送时也触发显示
        PREVENT_NEXT_SHOW.store(true, Ordering::Relaxed);
        if let Err(e) = input.input_key(enigo::Key::Return).await {
            log::error!("Failed to input return: {}", e);
        }
    });

    Ok(())
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
            .plugin(tauri_plugin_global_shortcut::Builder::new().build())
            .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
                let _ = handle::Handle::global().show_window();
            }))
            .setup(|app| {
                let main_window = app.get_webview_window("main").unwrap();
                main_window.set_shadow(true)?;
                main_window.set_size(LogicalSize::new(400.0, 80.0))?;
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

                // 启动游戏检测线程，使用轮询方式检测按键事件
                // 轮询是由于其他事件驱动的API在游戏环境都不太好使
                // 例如由于引擎Direct Input，系统按键事件无法监听
                // 全局快捷键会阻止按键发送到游戏本身
                tokio::spawn(async {
                    let game_detector = game_detector::GameDetector::new();

                    loop {
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
