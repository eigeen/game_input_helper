#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

#[cfg(target_os = "windows")]
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
use crate::settings::{AppSettings, SettingsStore};

#[cfg(target_os = "windows")]
const SETTINGS_RELOAD_INTERVAL: Duration = Duration::from_secs(1);

#[cfg(not(target_os = "windows"))]
pub struct GameDetector;

#[cfg(not(target_os = "windows"))]
impl GameDetector {
    pub fn new(_settings_store: crate::settings::SettingsStore) -> Self {
        Self
    }

    pub fn is_game_active(&self) -> bool {
        false
    }
}

#[cfg(target_os = "windows")]
pub struct GameDetector {
    settings_store: SettingsStore,
    settings: AppSettings,
    last_settings_load: Instant,
}

#[cfg(target_os = "windows")]
impl GameDetector {
    pub fn new(settings_store: SettingsStore) -> Self {
        Self {
            settings: load_settings_or_default(&settings_store),
            settings_store,
            last_settings_load: Instant::now(),
        }
    }

    pub fn is_game_active(&mut self) -> bool {
        self.reload_settings_if_needed();

        let title = match self.get_foreground_window_title() {
            Ok(title) => title,
            Err(error) => {
                log::debug!("failed to get foreground window title: {error}");
                return false;
            }
        };

        title_matches_keywords(&title, &self.settings.game_title_keywords)
    }

    pub fn get_foreground_window_title(&self) -> eyre::Result<String> {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_invalid() {
            return Err(eyre::eyre!("failed to get foreground window"));
        }

        window_title(hwnd)
    }

    fn reload_settings_if_needed(&mut self) {
        if self.last_settings_load.elapsed() < SETTINGS_RELOAD_INTERVAL {
            return;
        }

        self.settings = load_settings_or_default(&self.settings_store);
        self.last_settings_load = Instant::now();
    }
}

#[cfg(target_os = "windows")]
fn load_settings_or_default(settings_store: &SettingsStore) -> AppSettings {
    match settings_store.load() {
        Ok(settings) => settings,
        Err(error) => {
            log::debug!("failed to load settings: {error}");
            AppSettings::default()
        }
    }
}

#[cfg(target_os = "windows")]
fn title_matches_keywords(title: &str, keywords: &[String]) -> bool {
    let title_upper = title.to_uppercase();
    keywords
        .iter()
        .map(|keyword| keyword.to_uppercase())
        .any(|keyword| title_upper.contains(&keyword))
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
