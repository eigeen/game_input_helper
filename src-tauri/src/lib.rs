use std::{thread, time::Duration};

use tauri::{LogicalPosition, LogicalSize, Manager as _};

mod handle;
mod hotkey;
mod input;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn input(content: &str) {
    let handle = handle::Handle::global();
    let _ = handle.hide_window();

    thread::sleep(Duration::from_millis(100));
    let input = input::Input::global();
    if let Err(e) = input.input_key(enigo::Key::Return) {
        log::error!("Failed to input return: {}", e);
    }
    thread::sleep(Duration::from_millis(100));
    if let Err(e) = input.input_text(content) {
        log::error!("Failed to input text: {}", e);
    };
    // thread::sleep(Duration::from_millis(200));
    // if let Err(e) = input.input_key(enigo::Key::Return) {
    //     log::error!("Failed to input return: {}", e);
    // }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

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
            // main_window.set_resizable(false)?;
            // main_window.set_decorations(false)?;
            // main_window.set_skip_taskbar(true)?;

            handle::Handle::init(app.handle());
            input::Input::init().expect("Failed to initialize input");
            hotkey::Hotkey::global()
                .register("F7", hotkey::HotkeyFunc::SwitchDisplay)
                .expect("Failed to register hotkey");

            // main_window.clone().on_window_event(move |event| {
            //     if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            //         let _ = main_window.minimize();
            //         api.prevent_close();
            //     }
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![input])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(move |_handle, _event| {});
}
