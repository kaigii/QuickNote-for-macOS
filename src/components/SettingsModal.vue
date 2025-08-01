<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore, type Theme } from "../stores/settings";
import { useTranslation } from "i18next-vue";
import { invoke } from "@tauri-apps/api/tauri";

const { t, i18next } = useTranslation();
const settingsStore = useSettingsStore();

// Check if we're running in a web environment
const isWeb = typeof window !== 'undefined' && !window.__TAURI__;

// Local state for form values
const localTheme = ref<Theme>("light");
const localLanguage = ref("en");
const localDefaultFormat = ref("txt");
const localDefaultPath = ref("");
const localShortcutKeys = ref(["", "", ""]);
const localCloseTabShortcutKeys = ref(["", "", ""]);
const localToggleWindowShortcutKeys = ref(["", "", ""]);
const saveError = ref<string | null>(null);

// Emit close event
const emit = defineEmits<{
  close: [];
}>();

// Initialize local values when modal opens
onMounted(() => {
  localTheme.value = settingsStore.theme;
  localLanguage.value = settingsStore.language;
  localDefaultFormat.value = settingsStore.defaultFormat;
  localDefaultPath.value = settingsStore.defaultPath || "";
  localShortcutKeys.value = (settingsStore.newNoteShortcut || "").split("+").concat(["", "", ""]).slice(0, 3);
  localCloseTabShortcutKeys.value = (settingsStore.closeTabShortcut || "").split("+").concat(["", "", ""]).slice(0, 3);
  localToggleWindowShortcutKeys.value = (settingsStore.toggleWindowShortcut || "").split("+").concat(["", "", ""]).slice(0, 3);
});

const closeModal = () => {
  emit("close");
};

function handleKeyInput(event: KeyboardEvent, arr: string[], idx: number) {
  event.preventDefault();
  arr[idx] = "";
  const key = event.key;
  if (["Control", "Meta", "Alt", "Shift"].includes(key)) {
    arr[idx] =
      key === "Control" || key === "Meta"
        ? "CmdOrCtrl"
        : key === "Alt"
        ? "Option"
        : "Shift";
  } else {
    arr[idx] = key.length === 1 ? key.toUpperCase() : key;
  }
}

function getShortcutString(keys: string[]) {
  return keys.filter(Boolean).join("+");
}

const saveSettings = async () => {
  try {
    saveError.value = null;
    settingsStore.theme = localTheme.value;
    settingsStore.language = localLanguage.value;
    settingsStore.defaultFormat = localDefaultFormat.value;
    settingsStore.defaultPath = localDefaultPath.value || null;
    settingsStore.newNoteShortcut = getShortcutString(localShortcutKeys.value);
    settingsStore.closeTabShortcut = getShortcutString(localCloseTabShortcutKeys.value);
    settingsStore.toggleWindowShortcut = getShortcutString(localToggleWindowShortcutKeys.value);
    
    if (!isWeb) {
    await settingsStore.save_all_settings();
    }
    
    if (localLanguage.value !== i18next.language) {
      i18next.changeLanguage(localLanguage.value);
    }
    closeModal();
  } catch (error) {
    console.error("Failed to save settings:", error);
    saveError.value = `Failed to save settings. Please check permissions.`;
  }
};

const resetToDefaults = async () => {
  try {
    if (!isWeb) {
    await settingsStore.reset_settings();
    } else {
      // In web environment, just reset the local state
      settingsStore.theme = "light";
      settingsStore.language = "en";
      settingsStore.defaultFormat = "txt";
      settingsStore.defaultPath = null;
      settingsStore.newNoteShortcut = "CmdOrCtrl+Option+T";
      settingsStore.closeTabShortcut = "CmdOrCtrl+Option+Y";
      settingsStore.toggleWindowShortcut = "CmdOrCtrl+Option+U";
    }
    
    localTheme.value = settingsStore.theme;
    localLanguage.value = settingsStore.language;
    localDefaultFormat.value = settingsStore.defaultFormat;
    localDefaultPath.value = settingsStore.defaultPath || "";
    localShortcutKeys.value = (settingsStore.newNoteShortcut || "").split("+").concat(["", "", ""]).slice(0, 3);
    localCloseTabShortcutKeys.value = (settingsStore.closeTabShortcut || "").split("+").concat(["", "", ""]).slice(0, 3);
    localToggleWindowShortcutKeys.value = (settingsStore.toggleWindowShortcut || "").split("+").concat(["", "", ""]).slice(0, 3);
    if (settingsStore.language !== i18next.language) {
      i18next.changeLanguage(settingsStore.language);
    }
    closeModal();
  } catch (error) {
    console.error("Failed to reset settings:", error);
    saveError.value = `Failed to reset settings.`;
  }
};

const browsePath = async () => {
  if (isWeb) {
    // In web environment, show a message that file operations are not available
    alert("File operations are not available in the web version. Please download the desktop app for full functionality.");
    return;
  }

  try {
    const selectedPath = await invoke<string | null>("select_directory");
    if (selectedPath) {
      localDefaultPath.value = selectedPath;
    }
  } catch (error) {
    console.error("Failed to select directory:", error);
  }
};
</script>

<template>
  <div class="modal-overlay" @click="closeModal">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>{{ t("settings.title") }}</h2>
        <button class="close-btn" @click="closeModal">×</button>
      </div>

      <div class="modal-body">
        <div class="setting-item">
          <label for="theme">{{ t("settings.theme") }}</label>
          <select id="theme" v-model="localTheme">
            <option value="light">{{ t("settings.themeLight") }}</option>
            <option value="dark">{{ t("settings.themeDark") }}</option>
          </select>
        </div>

        <div class="setting-item">
          <label for="language">{{ t("settings.language") }}</label>
          <select id="language" v-model="localLanguage">
            <option value="en">English</option>
            <option value="zh-TW">繁體中文</option>
          </select>
        </div>

        <div class="setting-item">
          <label for="defaultFormat">{{ t("settings.defaultFormat") }}</label>
          <select id="defaultFormat" v-model="localDefaultFormat">
            <option value="txt">Plain Text</option>
            <option value="md">Markdown</option>
            <option value="js">JavaScript</option>
            <option value="ts">TypeScript</option>
            <option value="json">JSON</option>
            <option value="html">HTML</option>
            <option value="css">CSS</option>
          </select>
        </div>

        <div class="setting-item">
          <label for="defaultPath">{{ t("settings.defaultPath") }}</label>
          <div class="path-input-group">
            <input
              id="defaultPath"
              type="text"
              v-model="localDefaultPath"
              :placeholder="t('settings.defaultPathPlaceholder')"
            />
            <button class="browse-btn" @click="browsePath">
              {{ t("settings.browse") }}
            </button>
          </div>
        </div>

        <div class="setting-item">
          <label for="shortcut">{{ t("settings.newNoteShortcut") }}</label>
          <div class="shortcut-input-group">
            <input v-for="(key, idx) in localShortcutKeys" :key="idx" type="text" :value="key" @focus="localShortcutKeys[idx]=''" @keydown.prevent="handleKeyInput($event, localShortcutKeys, idx)" maxlength="12" />
          </div>
          <div class="shortcut-preview">{{ getShortcutString(localShortcutKeys) }}</div>
        </div>
        <div class="setting-item">
          <label for="closeTabShortcut">{{ t("settings.closeTabShortcut") }}</label>
          <div class="shortcut-input-group">
            <input v-for="(key, idx) in localCloseTabShortcutKeys" :key="idx" type="text" :value="key" @focus="localCloseTabShortcutKeys[idx]=''" @keydown.prevent="handleKeyInput($event, localCloseTabShortcutKeys, idx)" maxlength="12" />
          </div>
          <div class="shortcut-preview">{{ getShortcutString(localCloseTabShortcutKeys) }}</div>
        </div>
        <div class="setting-item">
          <label for="toggleWindowShortcut">{{ t("settings.toggleWindowShortcut") }}</label>
          <div class="shortcut-input-group">
            <input v-for="(key, idx) in localToggleWindowShortcutKeys" :key="idx" type="text" :value="key" @focus="localToggleWindowShortcutKeys[idx]=''" @keydown.prevent="handleKeyInput($event, localToggleWindowShortcutKeys, idx)" maxlength="12" />
          </div>
          <div class="shortcut-preview">{{ getShortcutString(localToggleWindowShortcutKeys) }}</div>
        </div>
      </div>

      <div class="modal-footer">
        <span v-if="saveError" class="error-message">{{ saveError }}</span>
        <button class="btn btn-secondary" @click="resetToDefaults">
          {{ t("settings.resetToDefaults") }}
        </button>
        <button class="btn btn-primary" @click="saveSettings">
          {{ t("settings.save") }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: var(--overlay-bg-color);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-color);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  width: 90%;
  max-width: 500px;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-color);
  overflow-x: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  color: var(--color-text-primary);
  font-size: 18px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-color);
  border-radius: 4px;
  padding: 0 8px;
  transition: background-color 0.2s;
  box-shadow: none;
}

.close-btn:hover {
  background-color: var(--color-background-secondary);
  color: var(--color-text-primary);
}

.modal-body {
  padding: 20px;
  overflow-y: auto;
  flex-grow: 1;
  overflow-x: hidden;
}

.setting-item {
  margin-bottom: 20px;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-item label {
  display: block;
  margin-bottom: 8px;
  color: var(--text-color);
  font-weight: 500;
}

.setting-item input[type="text"],
.setting-item select {
  width: 100%;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  background-color: var(--secondary-bg-color);
  color: var(--text-color);
  font-size: 14px;
  transition: border-color 0.2s;
  box-shadow: none !important;
}

.setting-item select {
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1.5em 1.5em;
  padding-right: 2.5rem;
}

.setting-item input[type="text"]:focus,
.setting-item select:focus {
  outline: none;
  border-color: var(--accent-color);
}

.path-input-group {
  display: flex;
  gap: 8px;
}

.path-input-group input {
  flex-grow: 1;
}

.browse-btn {
  padding: 8px 12px;
  background-color: var(--color-background-tertiary);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  color: var(--color-text-primary);
  cursor: pointer;
  white-space: nowrap;
}

.browse-btn:hover {
  background-color: var(--color-background-secondary);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding: 15px 20px;
  border-top: 1px solid var(--border-color);
  background-color: var(--tertiary-bg-color);
}

.error-message {
  color: #e53e3e;
  margin-right: auto;
  font-size: 0.9em;
}

.btn {
  border: 1px solid transparent;
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  transition:
    background-color 0.2s,
    border-color 0.2s;
  box-shadow: none;
}

.btn-secondary {
  background-color: var(--tertiary-bg-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
  margin-right: 10px;
}

.btn-secondary:hover {
  background-color: var(--border-color);
}

.btn-primary {
  background-color: var(--accent-color);
  color: var(--primary-btn-text-color);
  border: 1px solid var(--accent-color);
}

.btn-primary:hover {
  opacity: 0.9;
}

.shortcut-input-group {
  display: flex;
  gap: 8px;
  margin-bottom: 4px;
}
.shortcut-input-group input {
  width: 60px;
  text-align: center;
  font-size: 14px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--secondary-bg-color);
  color: var(--text-color);
  padding: 6px 0;
}
.shortcut-preview {
  color: var(--text-color-secondary);
  font-size: 13px;
  margin-bottom: 4px;
}
</style>
