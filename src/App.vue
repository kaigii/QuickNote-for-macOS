<script setup lang="ts">
import { ref, watchEffect, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import TabBar from "./components/TabBar.vue";
import EditorArea from "./components/EditorArea.vue";
import StatusBar from "./components/StatusBar.vue";
import SettingsModal from "./components/SettingsModal.vue";
import { useSettingsStore } from "./stores/settings";
import { useTabsStore } from "./stores/tabs";

const isSettingsOpen = ref(false);
const settingsStore = useSettingsStore();
const tabsStore = useTabsStore();

// Watch for theme changes and apply them to the body element
watchEffect(() => {
  document.body.dataset.theme = settingsStore.theme;
});

// Setup menu event listeners when the component mounts
onMounted(() => {
  tabsStore.setup_menu_listeners();
  
  // Listen for menu events from native menu and system tray
  listen("menu-event", (event) => {
    const menuId = event.payload as string;
    handleMenuEvent(menuId);
  });
});

const handleMenuEvent = (menuId: string) => {
  switch (menuId) {
    case "new_note":
      tabsStore.createTab();
      break;
    case "open_file":
      tabsStore.openFileFromDialog();
      break;
    case "save_file":
      tabsStore.saveActiveFile();
      break;
    case "save_file_as":
      tabsStore.saveActiveFileAs();
      break;
    case "close_active_tab":
      if (tabsStore.activeTabId) {
        tabsStore.closeTab(tabsStore.activeTabId);
      }
      break;
    case "toggle_theme":
      break;
  }
};

const openSettings = () => {
  isSettingsOpen.value = true;
};

const closeSettings = () => {
  isSettingsOpen.value = false;
};
</script>

<template>
  <div class="app-container">
    <TabBar @open-settings="openSettings" />
    <EditorArea />
    <StatusBar />
    <SettingsModal v-if="isSettingsOpen" @close="closeSettings" />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  border: 1px solid var(--border-color);
  border-radius: 12px;
  background-color: var(--bg-color);
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

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
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

#greet-input {
  margin-right: 5px;
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
}
</style>
