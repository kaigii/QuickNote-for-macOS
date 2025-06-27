import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

export type Theme = "light" | "dark";

export interface SettingsState {
  theme: Theme;
  language: string;
  defaultFormat: string;
  defaultPath: string | null;
  newNoteShortcut: string;
  closeTabShortcut: string;
  toggleWindowShortcut: string;
  loadError: boolean;
}

export const useSettingsStore = defineStore("settings", {
  state: (): SettingsState => ({
    theme: "light",
    language: "en",
    defaultFormat: "txt",
    defaultPath: null,
    newNoteShortcut: "CmdOrCtrl+Option+T",
    closeTabShortcut: "CmdOrCtrl+Option+Y",
    toggleWindowShortcut: "CmdOrCtrl+Option+U",
    loadError: false,
  }),

  actions: {
    /**
     * Loads settings from the backend and initializes the store.
     */
    async initialize_settings() {
      try {
        const settings = await invoke<{
          theme: string;
          language: string;
          default_format: string;
          default_path: string | null;
          new_note_shortcut: string;
          close_tab_shortcut: string;
          toggle_window_shortcut?: string;
        }>("load_settings");

        if (settings) {
          this.theme = settings.theme as Theme;
          this.language = settings.language;
          this.defaultFormat = settings.default_format;
          this.defaultPath = settings.default_path;
          this.newNoteShortcut = settings.new_note_shortcut;
          this.closeTabShortcut = settings.close_tab_shortcut;
          this.toggleWindowShortcut =
            settings.toggle_window_shortcut || "CmdOrCtrl+Option+U";
          this.loadError = false;
        }
      } catch (error) {
        console.error("Failed to load settings:", error);
        this.loadError = true;
      }
    },

    /**
     * Sets the application theme and persists it to the backend.
     * @param theme The theme to set.
     */
    setTheme: debounce(async function (
      this: SettingsState & { save_all_settings: () => Promise<void> },
      theme: Theme,
    ) {
      this.theme = theme;
      try {
        await invoke("save_settings", {
          settings: {
            theme: this.theme,
            language: this.language,
            default_format: this.defaultFormat,
            default_path: this.defaultPath,
            new_note_shortcut: this.newNoteShortcut,
            close_tab_shortcut: this.closeTabShortcut,
            toggle_window_shortcut: this.toggleWindowShortcut,
          },
        });
      } catch (error) {
        console.error("Failed to save settings:", error);
      }
    }, 300), // 300ms debounce

    /**
     * Saves all settings to the backend.
     */
    async save_all_settings() {
      try {
        await invoke("save_settings", {
          settings: {
            theme: this.theme,
            language: this.language,
            default_format: this.defaultFormat,
            default_path: this.defaultPath,
            new_note_shortcut: this.newNoteShortcut,
            close_tab_shortcut: this.closeTabShortcut,
            toggle_window_shortcut: this.toggleWindowShortcut,
          },
        });

        // Update the shortcuts after saving settings
        await invoke("update_shortcut", { shortcut: this.newNoteShortcut });
        await invoke("update_shortcut", {
          shortcut: this.closeTabShortcut,
          action: "close_tab",
        });
        await invoke("update_shortcut", {
          shortcut: this.toggleWindowShortcut,
          action: "toggle_window",
        });
      } catch (error) {
        console.error("Failed to save all settings:", error);
        throw error;
      }
    },

    /**
     * Resets all settings to default values and saves them.
     */
    async reset_settings() {
      this.theme = "light";
      this.language = "en";
      this.defaultFormat = "txt";
      this.defaultPath = null;
      this.newNoteShortcut = "CmdOrCtrl+Option+T";
      this.closeTabShortcut = "CmdOrCtrl+Option+Y";
      this.toggleWindowShortcut = "CmdOrCtrl+Option+U";

      await this.save_all_settings();
    },
  },
});

function debounce<T extends (...args: any[]) => any>(func: T, wait: number) {
  let timeout: ReturnType<typeof setTimeout> | null = null;
  return function (this: any, ...args: Parameters<T>) {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => {
      func.apply(this, args);
    }, wait);
  } as T;
}
