import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
  gameTitleKeywords: string[];
}

export function settingsToKeywordText(settings: AppSettings) {
  return settings.gameTitleKeywords.join("\n");
}

export function settingsFromKeywordText(keywordText: string): AppSettings {
  return {
    gameTitleKeywords: normalizeKeywordText(keywordText),
  };
}

export async function loadAppSettings() {
  return await invoke<AppSettings>("load_settings");
}

export async function saveAppSettings(settings: AppSettings) {
  await invoke("save_settings", { settings });
}

function normalizeKeywordText(keywordText: string) {
  return keywordText
    .split(/\r?\n/)
    .map((keyword) => keyword.trim())
    .filter((keyword) => keyword.length > 0);
}
