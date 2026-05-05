use crate::window_ime;

pub struct ImeGuard;

impl ImeGuard {
    pub fn new() -> Self {
        Self
    }

    pub fn keep_foreground_ime_english(&self) -> eyre::Result<()> {
        let hwnd = window_ime::foreground_window()?;
        window_ime::switch_window_to_english_layout(hwnd)?;
        window_ime::set_window_ime_to_english(hwnd);
        Ok(())
    }
}
