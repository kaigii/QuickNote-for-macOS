import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { ask } from "@tauri-apps/api/dialog";
import { useSettingsStore } from "./settings";
import i18next from "../i18n";
import { watch } from "vue";

/**
 * Represents a single tab in the editor.
 */
export interface Tab {
  id: string;
  name: string;
  path: string | null;
  content: string;
  cursorPos: number | null;
  isUnsaved: boolean;
}

// Helper to generate a unique ID for new tabs
const generateId = () => `tab_${Date.now()}_${Math.random()}`;

// Helper to get the next available "Untitled" number
const getNextUntitledNumber = (tabs: Tab[]): number => {
  // Use hardcoded prefixes to ensure reliability regardless of i18n loading state.
  const prefixes = ["Untitled", "未命名"];

  const untitledNumbers = tabs
    .map((tab) => {
      for (const prefix of prefixes) {
        const match = tab.name.match(new RegExp(`^${prefix}-(\\d+)$`));
        if (match) {
          return parseInt(match[1], 10);
        }
      }
      return 0;
    })
    .sort((a, b) => a - b);

  let nextNumber = 1;
  for (const number of untitledNumbers) {
    if (number === nextNumber) {
      nextNumber++;
    } else {
      break;
    }
  }
  return nextNumber;
};

// Helper function to extract filename from path
function extractFileName(path: string): string {
  return path.split(/[\\/]/).pop() || path;
}

// Check if we're running in a web environment
const isWeb = typeof window !== 'undefined' && !window.__TAURI__;

export const useTabsStore = defineStore("tabs", {
  state: () => {
    const firstTabId = generateId();
    return {
      tabs: [
        {
          id: firstTabId,
          name: `${i18next.t("tab.untitled_prefix")}-1`,
          path: null,
          content: "",
          cursorPos: null,
          isUnsaved: false,
        },
      ] as Tab[],
      activeTabId: firstTabId as string | null,
      recentlyClosed: [] as string[],
    };
  },

  getters: {
    /**
     * Gets the currently active tab object.
     */
    activeTab(state): Tab | undefined {
      return state.tabs.find((tab) => tab.id === state.activeTabId);
    },
  },

  actions: {
    /**
     * Creates a new, empty tab and sets it as the active one.
     */
    async createTab() {
      const newTab: Tab = {
        id: generateId(),
        name: `${i18next.t("tab.untitled_prefix")}-${getNextUntitledNumber(this.tabs)}`,
        path: null,
        content: "",
        cursorPos: 0,
        isUnsaved: false,
      };
      this.tabs.push(newTab);
      this.activeTabId = newTab.id;

      // Show and focus window when a new tab is created (Tauri only)
      if (!isWeb) {
      await appWindow.show();
      await appWindow.setFocus();
      }
    },

    /**
     * Closes a tab by its ID.
     * If the closed tab was active, it activates the next available tab.
     */
    async closeTab(tabId: string) {
      const tabIndex = this.tabs.findIndex((tab) => tab.id === tabId);
      if (tabIndex === -1) return;

      const tabToClose = this.tabs[tabIndex];

      // If tab has unsaved changes, ask user before closing
      if (tabToClose.isUnsaved) {
        if (isWeb) {
          // In web environment, use browser confirm dialog
          const result = confirm(`"${tabToClose.name}" has unsaved changes. Do you want to save?`);
          if (result) {
            const savedSuccessfully = await this.saveActiveFile();
            if (!savedSuccessfully) {
              return; // User cancelled the save dialog, so we abort closing the tab
            }
          }
        } else {
          // In Tauri environment, use native dialog
        const result = await ask(`"${tabToClose.name}" has unsaved changes.`, {
          title: "Unsaved Changes",
          type: "warning",
          okLabel: "Save",
          cancelLabel: "Don't Save", // This will return `false`
        });

        if (result) {
          // User clicked "Save", result is true
          const savedSuccessfully = await this.saveActiveFile();
          if (!savedSuccessfully) {
            return; // User cancelled the save dialog, so we abort closing the tab
            }
          }
        }
        // If user clicked "Don't Save" (result is false), we just proceed to close.
      }

      this._performCloseTab(tabId);
    },

    _performCloseTab(tabId: string) {
      const tabIndex = this.tabs.findIndex((tab) => tab.id === tabId);
      if (tabIndex === -1) return;

      const closedTab = this.tabs[tabIndex];

      // If we're closing the active tab, we need to find a new one to activate
      if (this.activeTabId === tabId) {
        if (this.tabs.length > 1) {
          const newActiveIndex =
            tabIndex === this.tabs.length - 1 ? tabIndex - 1 : tabIndex;
          this.activeTabId = this.tabs[newActiveIndex].id;
        } else {
          this.activeTabId = null;
        }
      }

      this.tabs.splice(tabIndex, 1);

      // Add closed file path to recently closed list
      if (closedTab.path) {
        this.recentlyClosed = this.recentlyClosed.filter(
          (path) => path !== closedTab.path,
        );
        this.recentlyClosed.unshift(closedTab.path!);
        if (this.recentlyClosed.length > 5) {
          this.recentlyClosed = this.recentlyClosed.slice(0, 5);
        }
      }

      // Hide window if no tabs left (Tauri only)
      if (!isWeb && this.tabs.length === 0) {
        appWindow.hide();
      }
    },

    /**
     * Selects a tab to be the active one.
     */
    selectTab(tabId: string) {
      if (this.tabs.some((tab) => tab.id === tabId)) {
        this.activeTabId = tabId;
      }
    },

    /**
     * Updates the content of the currently active tab and marks it as unsaved.
     */
    updateTabContent(content: string) {
      const activeTab = this.tabs.find((tab) => tab.id === this.activeTabId);
      if (activeTab) {
        activeTab.content = content;
        activeTab.isUnsaved = true;
      }
    },

    updateCursorPos(position: number) {
      const activeTab = this.tabs.find((tab) => tab.id === this.activeTabId);
      if (activeTab) {
        activeTab.cursorPos = position;
      }
    },

    // === File operations using backend commands ===
    async openFileFromDialog() {
      if (isWeb) {
        // In web environment, show a message that file operations are not available
        alert("File operations are not available in the web version. Please download the desktop app for full functionality.");
        return;
      }

      try {
        const results = await invoke<Array<{
          path: string;
          content: string;
        }> | null>("open_file");

        if (results && results.length > 0) {
          let lastTabId: string | null = null;
          for (const result of results) {
            const newTab: Tab = {
              id: generateId(),
              name: extractFileName(result.path),
              path: result.path,
              content: result.content,
              cursorPos: null,
              isUnsaved: false,
            };
            this.tabs.push(newTab);
            lastTabId = newTab.id;
          }

          if (lastTabId) {
            this.selectTab(lastTabId);
          }
        }
      } catch (error) {
        console.error("Failed to open file:", error);
      }
    },

    async openSpecificFile(path: string) {
      if (isWeb) {
        // In web environment, show a message that file operations are not available
        alert("File operations are not available in the web version. Please download the desktop app for full functionality.");
        return;
      }

      try {
        // 如果已經有這個檔案的分頁，直接切換
        const existingTab = this.tabs.find((tab) => tab.path === path);
        if (existingTab) {
          this.selectTab(existingTab.id);
          await appWindow.show();
          await appWindow.setFocus();
          return;
        }
        const result = await invoke<{ path: string; content: string } | null>(
          "open_specific_file",
          { path },
        );
        if (result) {
          const newTab: Tab = {
            id: generateId(),
            name: extractFileName(result.path),
            path: result.path,
            content: result.content,
            cursorPos: null,
            isUnsaved: false,
          };
          this.tabs.push(newTab);
          this.selectTab(newTab.id);

          // Show and focus window when opening a file
          await appWindow.show();
          await appWindow.setFocus();
        }
      } catch (error) {
        console.error("Failed to open specific file:", error);
      }
    },

    async saveActiveFile(): Promise<boolean> {
      const activeTab = this.tabs.find((tab) => tab.id === this.activeTabId);
      if (!activeTab) return false;

      // If no path, use save as
      if (!activeTab.path) {
        return this.saveActiveFileAs();
      }

      if (isWeb) {
        // In web environment, just mark as saved
        activeTab.isUnsaved = false;
        return true;
      }

      try {
        await invoke("save_file", {
          path: activeTab.path,
          content: activeTab.content,
        });
        // Mark as saved
        activeTab.isUnsaved = false;
        return true;
      } catch (error) {
        console.error("Failed to save file:", error);
        return false;
      }
    },

    async saveActiveFileAs(): Promise<boolean> {
      const activeTab = this.tabs.find((tab) => tab.id === this.activeTabId);
      if (!activeTab) return false;

      if (isWeb) {
        // In web environment, create a download link
        const blob = new Blob([activeTab.content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = activeTab.name || 'untitled.txt';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        
        // Mark as saved
        activeTab.isUnsaved = false;
        return true;
      }

      const settingsStore = useSettingsStore();

      try {
        const result = await invoke<string | null>("save_file_as", {
          content: activeTab.content,
          defaultFormat: settingsStore.defaultFormat,
          defaultName: activeTab.name,
        });
        if (result) {
          activeTab.path = result;
          activeTab.name = extractFileName(result);
          activeTab.isUnsaved = false;
          return true;
        }
        return false; // User cancelled the dialog
      } catch (error) {
        console.error("Failed to save file as:", error);
        return false;
      }
    },

    // === Menu event listeners ===
    setup_menu_listeners() {
      if (isWeb) {
        // Skip menu listeners in web environment
        return;
      }

      // Listen for shortcut-new-note event from backend
      listen("shortcut-new-note", () => {
        this.createTab();
      });

      // Listen for shortcut-close-tab event from backend
      listen("shortcut-close-tab", () => {
        if (this.activeTabId) {
          this.closeTab(this.activeTabId);
        }
      });

      // Listen for open-recent-file event from backend
      listen("open-recent-file", (event) => {
        const path = event.payload as string;
        this.openSpecificFile(path);
      });

      // Watch for changes in recentlyClosed array and update tray menu
      watch(
        () => this.recentlyClosed,
        async (newRecentFiles) => {
          try {
            await invoke("update_tray_menu", { recentFiles: newRecentFiles });
          } catch (error) {
            console.error("Failed to update tray menu:", error);
          }
        },
        { deep: true },
      );
    },

    moveTab(oldIndex: number, newIndex: number) {
      const [movedTab] = this.tabs.splice(oldIndex, 1);
      this.tabs.splice(newIndex, 0, movedTab);
    },

    /**
     * Renames a tab.
     */
    renameTab(tabId: string, newName: string) {
      const tab = this.tabs.find((t) => t.id === tabId);
      if (tab && newName.trim()) {
        tab.name = newName.trim();
        tab.isUnsaved = true; // Renaming should mark the tab as unsaved
      }
    },
  },
});
