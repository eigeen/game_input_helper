use crate::handle;
use std::sync::OnceLock;
use tauri_plugin_global_shortcut::{GlobalShortcutExt as _, ShortcutState};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotkeyFunc {
    SwitchDisplay,
}
