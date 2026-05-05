use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

const SETTINGS_FILE_NAME: &str = "game_input_helper.settings.json";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub game_title_keywords: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            game_title_keywords: default_game_title_keywords(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    pub fn for_app_directory() -> eyre::Result<Self> {
        Ok(Self::new(settings_file_path()?))
    }

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> eyre::Result<AppSettings> {
        if !self.path.exists() {
            return self.save_default_settings();
        }

        read_settings_file(&self.path)
    }

    pub fn save(&self, settings: AppSettings) -> eyre::Result<()> {
        let content = serde_json::to_string_pretty(&normalize_settings(settings))?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    fn save_default_settings(&self) -> eyre::Result<AppSettings> {
        let settings = AppSettings::default();
        self.save(settings.clone())?;
        Ok(settings)
    }
}

fn settings_file_path() -> eyre::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let app_dir = exe_path
        .parent()
        .ok_or_else(|| eyre::eyre!("failed to resolve app directory"))?;
    Ok(app_dir.join(SETTINGS_FILE_NAME))
}

fn read_settings_file(path: &PathBuf) -> eyre::Result<AppSettings> {
    let content = fs::read_to_string(path)?;
    let settings = serde_json::from_str::<AppSettings>(&content)?;
    Ok(normalize_settings(settings))
}

fn normalize_settings(settings: AppSettings) -> AppSettings {
    let keywords = normalize_keywords(settings.game_title_keywords);
    AppSettings {
        game_title_keywords: fallback_keywords(keywords),
    }
}

fn normalize_keywords(keywords: Vec<String>) -> Vec<String> {
    keywords
        .into_iter()
        .map(|keyword| keyword.trim().to_string())
        .filter(|keyword| !keyword.is_empty())
        .collect()
}

fn fallback_keywords(keywords: Vec<String>) -> Vec<String> {
    if keywords.is_empty() {
        return default_game_title_keywords();
    }

    keywords
}

fn default_game_title_keywords() -> Vec<String> {
    #[cfg(debug_assertions)]
    return vec!["HELLDIVERS".to_string(), "VISUAL STUDIO CODE".to_string()];

    #[cfg(not(debug_assertions))]
    return vec!["HELLDIVERS".to_string()];
}
