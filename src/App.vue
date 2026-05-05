<script setup lang="ts">
import { onMounted, ref, useTemplateRef } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen as tauriListen } from "@tauri-apps/api/event";
import { openUrl } from "@tauri-apps/plugin-opener";
import { InputHistory } from "./inputHistory";

const content = ref("");
const inputHistory = new InputHistory();

const inputBox = useTemplateRef("inputBox");

async function input_submit() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  const submittedContent = content.value;

  try {
    await invoke("input", { content: submittedContent });
    inputHistory.recordSubmittedContent(submittedContent);
    content.value = "";
  } catch (e) {
    console.error(e);
  }
}

function show_older_history_content() {
  content.value = inputHistory.showOlderContent(content.value);
}

function show_newer_history_content() {
  content.value = inputHistory.showNewerContent(content.value);
}

function set_input_focus() {
  if (inputBox.value) {
    inputBox.value.focus();
  }
}

async function open_settings_window() {
  try {
    await invoke("open_settings_window");
  } catch (e) {
    console.error(e);
  }
}

onMounted(() => {
  tauriListen("focus_input", (_event) => {
    set_input_focus();
  });
  set_input_focus();
});
</script>

<template>
  <main class="container">
    <form
      class="row"
      @submit.prevent="input_submit"
      @keydown.up.prevent="show_older_history_content"
      @keydown.down.prevent="show_newer_history_content"
    >
      <input id="input-box" ref="inputBox" v-model="content" placeholder="输入内容..." />
      <button class="submit-button" type="submit">按Enter发送</button>
      <i
        class="mdi mdi-help-circle-outline icon"
        title="GitHub"
        @click="openUrl('https://github.com/eigeen/game_input_helper')"
      ></i>
      <i class="mdi mdi-cog-outline icon" title="设置" @click="open_settings_window"></i>
    </form>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  overflow: hidden;
}

.container {
  margin: 0;
  /* padding-top: 10vh; */
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  box-sizing: border-box;
  display: grid;
  grid-template-columns: minmax(0, 1fr) 104px 28px 28px;
  align-items: center;
  gap: 4px;
  width: 100vw;
  height: 70px;
  padding: 8px;
}

input,
button {
  box-sizing: border-box;
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

.submit-button {
  box-sizing: border-box;
  width: 104px;
  height: 54px;
  padding: 0;
  line-height: 20px;
  white-space: nowrap;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#input-box {
  width: 100%;
  min-width: 0;
  height: 54px;
}

.icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 54px;
  color: #666;
  cursor: pointer;
  font-size: 20px;
  transition: color 0.2s ease;
}

.icon:hover {
  color: #396cd8;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }

  .icon {
    color: #ccc;
  }

  .icon:hover {
    color: #24c8db;
  }
}
</style>
