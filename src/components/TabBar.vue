<script setup lang="ts">
import { onMounted, ref, nextTick, computed } from "vue";
import { useTabsStore, type Tab } from "../stores/tabs";
import Sortable from "sortablejs";
import { appWindow } from "@tauri-apps/api/window";

// Import Vue and utility functions

// Check if we're running in a web environment
const isWeb = typeof window !== 'undefined' && !window.__TAURI__;

// Store reference to the tabs state
const tabsStore = useTabsStore();
const tabsContainer = ref<HTMLDivElement | null>(null);
const editingTabId = ref<string | null>(null);
const originalName = ref("");

// macOS detection for traffic light UI
const isMac = computed(() => navigator.userAgent.includes("Macintosh"));

// State for traffic light button hover/press
const closeState = ref<"normal" | "hover" | "press">("normal");
const minimizeState = ref<"normal" | "hover" | "press">("normal");
const maximizeState = ref<"normal" | "hover" | "press">("normal");

// Functions to control group hover state for traffic lights
const setAllHover = () => {
  closeState.value = "hover";
  minimizeState.value = "hover";
  maximizeState.value = "hover";
};
const setAllNormal = () => {
  closeState.value = "normal";
  minimizeState.value = "normal";
  maximizeState.value = "normal";
};

// Get SVG path for traffic light buttons
const trafficLightSvg = (
  type: "close" | "minimize" | "maximize",
  state: "normal" | "hover" | "press",
) => {
  if (type === "minimize" || type === "maximize") {
    return "/traffic-lights/all-three-nofocus.svg";
  }
  return `/traffic-lights/${type}-${state}.svg`;
};

// Only close (red) button has click handler; minimize and maximize are UI only
const handleClose = () => {
  if (!isWeb) {
    appWindow.hide();
  } else {
    // In web environment, show a message
    alert("This is a web demo. Please download the desktop app for full functionality.");
  }
};

// Emit events for parent communication
const emit = defineEmits<{
  "open-settings": [];
}>();

// Setup sortable for tab drag-and-drop
onMounted(() => {
  if (tabsContainer.value) {
    Sortable.create(tabsContainer.value, {
      animation: 150,
      onEnd: (evt) => {
        if (evt.oldIndex !== undefined && evt.newIndex !== undefined) {
          tabsStore.moveTab(evt.oldIndex, evt.newIndex);
        }
      },
    });
  }
});

// Tab renaming logic
const startEditing = async (tab: Tab) => {
  editingTabId.value = tab.id;
  originalName.value = tab.name;
  await nextTick();
  const input = document.getElementById(
    `tab-input-${tab.id}`,
  ) as HTMLInputElement;
  if (input) {
    input.focus();
    input.select();
  }
};

const finishEditing = (tab: Tab, event: Event) => {
  if (editingTabId.value !== tab.id) return;
  const newName = (event.target as HTMLInputElement).value.trim();
  if (newName) {
    tabsStore.renameTab(tab.id, newName);
  }
  editingTabId.value = null;
};

const cancelEditing = () => {
  editingTabId.value = null;
};

// Open settings modal
const openSettings = () => {
  emit("open-settings");
};
</script>

<template>
  <div class="tab-bar">
    <div class="top-drag-handle" data-tauri-drag-region>
      <img
        class="drag-indicator"
        src="/drag-indicator.svg"
        alt="drag indicator"
      />
    </div>
    <div
      v-if="isMac"
      class="traffic-light-group"
      @mouseenter="setAllHover"
      @mouseleave="setAllNormal"
    >
      <div
        class="traffic-light-btn close"
        :style="{
          backgroundImage: `url('${trafficLightSvg('close', closeState)}')`,
        }"
        @mousedown="closeState = 'press'"
        @mouseup="closeState = 'hover'"
        @click="handleClose"
        tabindex="-1"
        role="button"
        aria-label="Close"
      ></div>
      <div
        class="traffic-light-btn minimize"
        :style="{
          backgroundImage: `url('${trafficLightSvg('minimize', minimizeState)}')`,
        }"
        @mousedown="minimizeState = 'press'"
        @mouseup="minimizeState = 'hover'"
        tabindex="-1"
        role="button"
        aria-label="Minimize"
      ></div>
      <div
        class="traffic-light-btn maximize"
        :style="{
          backgroundImage: `url('${trafficLightSvg('maximize', maximizeState)}')`,
        }"
        @mousedown="maximizeState = 'press'"
        @mouseup="maximizeState = 'hover'"
        tabindex="-1"
        role="button"
        aria-label="Maximize"
      ></div>
    </div>
    <div class="tabs-container" ref="tabsContainer">
      <div
        v-for="tab in tabsStore.tabs"
        :key="tab.id"
        class="tab-item"
        :class="{ active: tab.id === tabsStore.activeTabId }"
        @click="tabsStore.selectTab(tab.id)"
        @dblclick="startEditing(tab)"
      >
        <input
          v-if="editingTabId === tab.id"
          :id="`tab-input-${tab.id}`"
          type="text"
          :value="tab.name"
          @blur="finishEditing(tab, $event)"
          @keydown.enter="finishEditing(tab, $event)"
          @keydown.esc="cancelEditing"
          class="tab-rename-input"
          @click.stop
        />
        <span v-else class="tab-title"
          >{{ tab.name }}{{ tab.isUnsaved ? " •" : "" }}</span
        >
        <button class="close-tab-btn" @click.stop="tabsStore.closeTab(tab.id)">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M6 18 18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>
    <div class="controls">
      <button
        class="action-btn"
        id="add-tab-btn"
        @click="tabsStore.createTab()"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M12 4.5v15m7.5-7.5h-15"
          />
        </svg>
      </button>
      <button class="action-btn" id="settings-btn" @click="openSettings">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 0 1 1.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.559.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.894.149c-.424.07-.764.383-.929.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 0 1-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.398.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 0 1-.12-1.45l.527-.737c.25-.35.272-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.108-1.204l-.526-.738a1.125 1.125 0 0 1 .12-1.45l.773-.773a1.125 1.125 0 0 1 1.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894Z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.tab-bar {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  background-color: var(--bg-color);
  border-bottom: none;
  height: 40px;
  padding: 0 0 0 10px;
  flex-shrink: 0;
  user-select: none;
  position: relative;
  color: var(--text-color);
}

.tab-bar::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 1px;
  background: var(--border-color);
  z-index: 1;
}

.top-drag-handle {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 33%;
  background-color: var(--drag-handle-bg);
  transform: translateY(-100%);
  transition: transform 0.2s ease-in-out;
  pointer-events: auto;
  z-index: 2;
  -webkit-app-region: drag;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.tab-bar:hover .top-drag-handle {
  transform: translateY(0);
}

.drag-indicator {
  width: 14px;
  height: 14px;
  opacity: 0.7;
  pointer-events: none;
  filter: brightness(0) saturate(100%) invert(69%) sepia(97%) saturate(749%)
    hue-rotate(176deg) brightness(98%) contrast(101%);
}

.traffic-light-group {
  display: flex;
  align-items: center;
  height: 100%;
  margin-left: 0;
  margin-right: 12px;
  gap: 8px;
  -webkit-app-region: no-drag;
}
.traffic-light-btn {
  width: 12px;
  height: 12px;
  min-width: 12px;
  min-height: 12px;
  background-size: 100% 100%;
  background-repeat: no-repeat;
  background-position: center;
  border-radius: 50%;
  cursor: pointer;
  margin-top: 0;
  margin-bottom: 0;
  display: inline-block;
  -webkit-app-region: no-drag;
  image-rendering: auto;
}
.tabs-container {
  display: flex;
  flex-grow: 1;
  height: 100%;
  overflow-x: auto;
  overflow-y: hidden;
}

.tabs-container::-webkit-scrollbar {
  height: 4px;
  background-color: transparent;
}

.tabs-container::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 2px;
}

.tabs-container:hover::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.4);
}

.tab-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 15px;
  border-right: 1px solid var(--border-color);
  cursor: pointer;
  gap: 10px;
  white-space: nowrap;
}

.tab-item:hover {
  background-color: var(--tertiary-bg-color);
}

.tab-item.active {
  background-color: var(--secondary-bg-color);
  color: var(--text-color);
  position: relative;
  border-bottom: 0 !important;
  z-index: 2;
}
.tab-item.active::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  bottom: -1px;
  height: 2px;
  background: var(--secondary-bg-color);
  z-index: 3;
}

.close-tab-btn {
  background: none;
  border: none;
  box-shadow: none;
  color: var(--text-color);
  cursor: pointer;
  padding: 0;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.close-tab-btn svg {
  width: 16px;
  height: 16px;
}

.close-tab-btn:hover {
  background-color: var(--border-color);
}

.controls {
  display: flex;
  gap: 5px;
  padding-right: 10px;
}

.action-btn {
  background: none;
  border: none;
  box-shadow: none;
  color: var(--text-color);
  cursor: pointer;
  padding: 5px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn svg {
  width: 20px;
  height: 20px;
}

.action-btn:hover {
  background-color: var(--tertiary-bg-color);
}

.tab-rename-input {
  background-color: var(--bg-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
  outline: none;
  padding: 2px 5px;
  border-radius: 3px;
  width: 120px; /* Adjust width as needed */
  font-family: inherit;
  font-size: inherit;
}

.tab-title {
  user-select: none;
}
</style>
