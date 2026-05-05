<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import {
  loadAppSettings,
  saveAppSettings,
  settingsFromKeywordText,
  settingsToKeywordText,
} from "./settingsApi";

const keywordText = ref("");
const statusText = ref("");
const isLoading = ref(false);
const isSaving = ref(false);

const canSave = computed(() => {
  return !isLoading.value && !isSaving.value && keywordText.value.trim().length > 0;
});

async function loadSettings() {
  isLoading.value = true;
  statusText.value = "";

  try {
    keywordText.value = settingsToKeywordText(await loadAppSettings());
  } catch (error) {
    statusText.value = `加载失败：${error}`;
  } finally {
    isLoading.value = false;
  }
}

async function saveSettings() {
  if (!canSave.value) {
    return;
  }

  isSaving.value = true;
  statusText.value = "";

  try {
    await saveAppSettings(settingsFromKeywordText(keywordText.value));
    statusText.value = "已保存";
  } catch (error) {
    statusText.value = `保存失败：${error}`;
  } finally {
    isSaving.value = false;
  }
}

function resetKeywords() {
  keywordText.value = "HELLDIVERS";
  statusText.value = "";
}

onMounted(loadSettings);
</script>

<template>
  <main class="settings-shell">
    <header class="titlebar">
      <div>
        <h1>设置</h1>
        <p>game_input_helper.settings.json</p>
      </div>
      <button class="icon-button" type="button" title="重新加载" @click="loadSettings">
        <i class="mdi mdi-refresh"></i>
      </button>
    </header>

    <section class="setting-block">
      <label for="game-title-keywords">游戏窗口标题关键词</label>
      <textarea
        id="game-title-keywords"
        v-model="keywordText"
        spellcheck="false"
        placeholder="HELLDIVERS"
      ></textarea>
    </section>

    <footer class="actions">
      <span class="status">{{ statusText }}</span>
      <button class="secondary-button" type="button" @click="resetKeywords">重置</button>
      <button class="primary-button" type="button" :disabled="!canSave" @click="saveSettings">
        保存
      </button>
    </footer>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  color: #16181d;
  background: #f4f6f8;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

body {
  margin: 0;
}

button,
textarea {
  font: inherit;
}

.settings-shell {
  box-sizing: border-box;
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 18px;
  min-height: 100vh;
  padding: 22px;
}

.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.titlebar h1 {
  margin: 0;
  font-size: 22px;
  font-weight: 650;
}

.titlebar p {
  margin: 2px 0 0;
  color: #687080;
  font-size: 13px;
}

.icon-button {
  width: 36px;
  height: 36px;
  border: 1px solid #cfd6df;
  border-radius: 8px;
  color: #2f5f9f;
  background: #ffffff;
  cursor: pointer;
}

.setting-block {
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 8px;
  min-height: 0;
}

.setting-block label {
  font-size: 14px;
  font-weight: 600;
}

.setting-block textarea {
  box-sizing: border-box;
  width: 100%;
  min-height: 220px;
  resize: none;
  border: 1px solid #cfd6df;
  border-radius: 8px;
  padding: 12px;
  color: #16181d;
  background: #ffffff;
  outline: none;
}

.setting-block textarea:focus {
  border-color: #2f5f9f;
  box-shadow: 0 0 0 3px rgba(47, 95, 159, 0.14);
}

.actions {
  display: grid;
  grid-template-columns: 1fr auto auto;
  align-items: center;
  gap: 10px;
}

.status {
  min-width: 0;
  overflow: hidden;
  color: #687080;
  font-size: 13px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.primary-button,
.secondary-button {
  min-width: 76px;
  height: 36px;
  border-radius: 8px;
  border: 1px solid #cfd6df;
  cursor: pointer;
}

.primary-button {
  border-color: #2f5f9f;
  color: #ffffff;
  background: #2f5f9f;
}

.primary-button:disabled {
  cursor: default;
  opacity: 0.5;
}

.secondary-button {
  color: #273142;
  background: #ffffff;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f2f5f8;
    background: #24282f;
  }

  .titlebar p,
  .status {
    color: #aab4c0;
  }

  .icon-button,
  .secondary-button,
  .setting-block textarea {
    border-color: #454d59;
    color: #f2f5f8;
    background: #171a1f;
  }

  .icon-button {
    color: #8bb7ff;
  }
}
</style>
