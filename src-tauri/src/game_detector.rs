#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

#[cfg(not(debug_assertions))]
const GAME_TITLES: &[&str] = &["HELLDIVERS"];
#[cfg(debug_assertions)]
const GAME_TITLES: &[&str] = &["HELLDIVERS", "VISUAL STUDIO CODE"];

#[cfg(not(target_os = "windows"))]
pub struct GameDetector;

#[cfg(not(target_os = "windows"))]
impl GameDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn is_game_active(&self) -> bool {
        false
    }
}

#[cfg(target_os = "windows")]
pub struct GameDetector;

#[cfg(target_os = "windows")]
impl GameDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn is_game_active(&self) -> bool {
        let title = match self.get_foreground_window_title() {
            Ok(title) => title,
            Err(error) => {
                log::debug!("failed to get foreground window title: {error}");
                return false;
            }
        };

        let title_upper = title.to_uppercase();
        GAME_TITLES
            .iter()
            .any(|&game_title| title_upper.contains(game_title))
    }

    pub fn get_foreground_window_title(&self) -> eyre::Result<String> {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_invalid() {
            return Err(eyre::eyre!("failed to get foreground window"));
        }

        window_title(hwnd)
    }
}

#[cfg(target_os = "windows")]
fn window_title(hwnd: windows::Win32::Foundation::HWND) -> eyre::Result<String> {
    let mut window_text: [u16; 512] = [0; 512];
    let length = unsafe { GetWindowTextW(hwnd, &mut window_text) };
    if length == 0 {
        return Err(eyre::eyre!("failed to get foreground window title"));
    }

    Ok(String::from_utf16_lossy(&window_text[..length as usize]))
}
