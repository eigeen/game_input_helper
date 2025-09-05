use log::debug;

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

#[cfg(not(target_os = "windows"))]
pub struct GameDetector;

#[cfg(not(target_os = "windows"))]
impl GameDetector {
    pub fn new() -> Self {
        GameDetector
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
        GameDetector
    }

    pub fn is_game_active(&self) -> bool {
        let title = match self.get_foreground_window_title() {
            Ok(title) => title,
            Err(e) => {
                debug!("获取窗口标题失败: {}", e);
                return false;
            }
        };
        debug!("前台窗口标题: {}", title);

        // 检查是否包含HELLDIVERS关键字
        title.to_uppercase().contains("HELLDIVERS")
        // title.to_uppercase().contains("TRAE")
    }

    pub fn get_foreground_window_title(&self) -> eyre::Result<String> {
        #[cfg(target_os = "windows")]
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_invalid() {
                return Err(eyre::eyre!("获取前台窗口句柄失败"));
            }

            let mut window_text: [u16; 512] = [0; 512];
            let length = GetWindowTextW(hwnd, &mut window_text);

            if length == 0 {
                return Err(eyre::eyre!("获取窗口标题失败"));
            }

            Ok(String::from_utf16_lossy(&window_text[..length as usize]))
        }

        #[cfg(not(target_os = "windows"))]
        Err(eyre::eyre!("非Windows系统"))
    }
}
