use crate::window_ime::{self, ImeSnapshot, WindowHandle};
use std::time::{Duration, Instant};

const RESTORE_SETTLE_DELAY: Duration = Duration::from_millis(200);

pub struct AppImeState {
    focused: bool,
    next_save_at: Instant,
    saved_snapshot: Option<ImeSnapshot>,
}

impl AppImeState {
    pub fn new() -> Self {
        Self {
            focused: false,
            next_save_at: Instant::now(),
            saved_snapshot: None,
        }
    }

    pub fn update(&mut self, window: &tauri::WebviewWindow) -> eyre::Result<()> {
        let focused = window.is_focused()?;
        let hwnd = app_window_handle(window)?;

        if focused && !self.focused && self.restore_saved_snapshot(hwnd)? {
            self.next_save_at = Instant::now() + RESTORE_SETTLE_DELAY;
        }

        self.focused = focused;
        if focused && Instant::now() >= self.next_save_at {
            self.save_current_snapshot(hwnd)?;
        }

        Ok(())
    }

    fn restore_saved_snapshot(&self, hwnd: WindowHandle) -> eyre::Result<bool> {
        let Some(snapshot) = self.saved_snapshot else {
            return Ok(false);
        };

        window_ime::restore_window_ime(hwnd, snapshot)?;
        Ok(true)
    }

    fn save_current_snapshot(&mut self, hwnd: WindowHandle) -> eyre::Result<()> {
        self.saved_snapshot = Some(window_ime::capture_window_ime(hwnd)?);
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn app_window_handle(window: &tauri::WebviewWindow) -> eyre::Result<WindowHandle> {
    Ok(window_ime::window_handle_from_ptr(window.hwnd()?.0 as _))
}

#[cfg(not(target_os = "windows"))]
fn app_window_handle(_window: &tauri::WebviewWindow) -> eyre::Result<WindowHandle> {
    Ok(())
}
