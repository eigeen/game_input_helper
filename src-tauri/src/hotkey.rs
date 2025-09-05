use std::sync::OnceLock;

use tauri_plugin_global_shortcut::{GlobalShortcutExt as _, ShortcutState};
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};

use crate::handle;

pub struct Hotkey {}

impl Hotkey {
    pub fn global() -> &'static Self {
        static HOTKEY: OnceLock<Hotkey> = OnceLock::new();

        HOTKEY.get_or_init(|| Hotkey {})
    }

    pub fn register(&self, hotkey: &str, func_type: HotkeyFunc) -> eyre::Result<()> {
        let app_handle = handle::Handle::global().app_handle().unwrap();
        let manager = app_handle.global_shortcut();

        let _ = manager.on_shortcut(hotkey, move |_app_handle, _hotkey, event| {
            if event.state() == ShortcutState::Pressed {
                match func_type {
                    HotkeyFunc::SwitchDisplay => {
                        let handle = handle::Handle::global();
                        let is_showing = *handle.is_showing.lock();
                        if is_showing {
                            let _ = handle.hide_window();
                        } else {
                            let _ = handle.show_window();
                        }
                    }
                }
            }
        });

        log::debug!("Registered hotkey: {hotkey} -> {func_type:?}");

        Ok(())
    }

    /// 自上次检查以来是否按下了键
    #[cfg(target_os = "windows")]
    pub fn is_key_pressed_async(&self, vk_code: VIRTUAL_KEY) -> bool {
        unsafe {
            let result = GetAsyncKeyState(vk_code.0 as i32);
            // 自上次检查以来按下了键
            // (result as u16 & 0x8000) != 0
            result as u16 & 0x1 != 0
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_key_pressed_async(&self, _vk_code: i32) -> bool {
        false // 非Windows系统不支持
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotkeyFunc {
    SwitchDisplay,
}
