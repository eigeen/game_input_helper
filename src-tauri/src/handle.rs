use std::sync::OnceLock;

use parking_lot::{Mutex, RwLock};
use tauri::{AppHandle, Emitter as _, Manager};

pub struct Handle {
    app_handle: RwLock<Option<AppHandle>>,
    pub(crate) is_showing: Mutex<bool>,
}

impl Handle {
    pub fn global() -> &'static Handle {
        static HANDLE: OnceLock<Handle> = OnceLock::new();

        HANDLE.get_or_init(|| Handle {
            app_handle: RwLock::new(None),
            is_showing: Mutex::new(true),
        })
    }

    pub fn init(app_handle: &AppHandle) {
        let mut handle = Self::global().app_handle.write();
        *handle = Some(app_handle.clone());
    }

    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }

    pub fn get_main_window(&self) -> Option<tauri::WebviewWindow> {
        let app_handle = self.app_handle()?;
        let webview = app_handle.get_webview_window("main")?;
        Some(webview)
    }

    pub fn show_window(&self) -> eyre::Result<()> {
        let main_window = self.get_main_window().unwrap();

        if *self.is_showing.lock() {
            let _ = main_window.set_focus();
            return Ok(());
        }

        let _ = main_window.show();
        let _ = main_window.set_focus();
        *self.is_showing.lock() = true;
        let _ = self
            .app_handle
            .read()
            .as_ref()
            .unwrap()
            .emit("focus_input", ());

        Ok(())
    }

    pub fn hide_window(&self) -> eyre::Result<()> {
        if !*self.is_showing.lock() {
            return Ok(());
        }

        let main_window = self.get_main_window().unwrap();
        let _ = main_window.hide();
        *self.is_showing.lock() = false;

        Ok(())
    }
}
