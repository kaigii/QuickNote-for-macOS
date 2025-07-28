<script setup lang="ts">
import {
  computed,
  ref,
  watch,
  onMounted,
  onBeforeUnmount,
  nextTick,
} from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useTabsStore } from "../stores/tabs";
import { useTranslation } from "i18next-vue";
import { debounce } from "lodash-es";

const { t } = useTranslation();
const tabsStore = useTabsStore();
const editorRef = ref<HTMLTextAreaElement | null>(null);
const lineNumbersRef = ref<HTMLDivElement | null>(null);
let unlisten: UnlistenFn | null = null;

// Check if we're running in a web environment
const isWeb = typeof window !== 'undefined' && !window.__TAURI__;

const activeTab = computed(() => tabsStore.activeTab);

const lineCount = computed(() => {
  return activeTab.value ? activeTab.value.content.split("\n").length : 1;
});

const debouncedUpdateContent = debounce((val: string) => {
  tabsStore.updateTabContent(val);
}, 100);

const handleInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  debouncedUpdateContent(target.value);
};

const syncScroll = () => {
  if (editorRef.value && lineNumbersRef.value) {
    lineNumbersRef.value.scrollTop = editorRef.value.scrollTop;
  }
};

onMounted(async () => {
  // Listen for the custom file drop event from the Rust backend (Tauri only)
  if (!isWeb) {
    unlisten = await listen<string[]>("custom-file-drop", (event) => {
      for (const filePath of event.payload) {
        tabsStore.openSpecificFile(filePath);
      }
    });
  }
});

onBeforeUnmount(() => {
  // Clean up the listener when the component is unmounted
  if (unlisten) {
    unlisten();
  }
});

// Watch for active tab changes to focus, set cursor and sync scroll
watch(
  activeTab,
  (newTab) => {
  if (newTab) {
    nextTick(() => {
      if (editorRef.value) {
        editorRef.value.focus();
        if (newTab.cursorPos !== null) {
          editorRef.value.selectionStart = newTab.cursorPos;
          editorRef.value.selectionEnd = newTab.cursorPos;
        }
        // When tab changes, sync scroll position
        syncScroll();
      }
    });
  }
  },
  { immediate: true },
);

// Watch for content changes specifically to reset scroll on new file load
watch(
  () => activeTab.value?.content,
  () => {
    nextTick(() => {
        if (editorRef.value) {
            syncScroll();
        }
    });
  },
);

const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === "Tab") {
        event.preventDefault();
        const target = event.target as HTMLTextAreaElement;
        const start = target.selectionStart;
        const end = target.selectionEnd;

    target.value =
      target.value.substring(0, start) + "\t" + target.value.substring(end);
        target.selectionStart = target.selectionEnd = start + 1;
        
        tabsStore.updateTabContent(target.value);
    }
};

const handleCursorActivity = () => {
    if (editorRef.value) {
        tabsStore.updateCursorPos(editorRef.value.selectionStart);
    }
};
</script>

<template>
  <div class="editor-area">
    <div v-if="activeTab" class="editor-container">
      <div class="line-numbers" ref="lineNumbersRef">
        <div v-for="n in lineCount" :key="n">{{ n }}</div>
      </div>
      <textarea
        ref="editorRef"
        :value="activeTab.content"
        class="editor"
        :placeholder="t('editor.placeholder')"
        spellcheck="false"
        wrap="off"
        @input="handleInput"
        @keydown="handleKeyDown"
        @scroll="syncScroll"
        @click="handleCursorActivity"
        @keyup="handleCursorActivity"
        @selectionchange="handleCursorActivity"
      ></textarea>
    </div>
    <div v-else class="placeholder-container">
      <span>{{ t("editor.noTabMessage") }}</span>
    </div>
  </div>
</template>

<style scoped>
.editor-area {
  flex-grow: 1;
  display: flex;
  background-color: var(--secondary-bg-color);
  overflow: hidden; /* Prevent the main container from scrolling */
}

.editor-container {
  display: flex;
  flex-grow: 1;
  height: 100%;
}

.line-numbers {
  width: 50px;
  padding: 10px 0;
  padding-left: 10px;
  text-align: right;
  background-color: var(--secondary-bg-color);
  color: var(--text-color);
  font-family:
    Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 14px;
  line-height: calc(14px * 1.5);
  border-right: 1px solid var(--border-color);
  user-select: none;
  overflow-y: hidden; /* Hide the native scrollbar */
}

.line-numbers div {
  display: block;
  height: calc(14px * 1.5); /* Match editor line-height */
  padding-right: 10px;
}

.editor {
  flex-grow: 1;
  width: 100%;
  height: 100%;
  padding: 10px;
  border: none;
  outline: none;
  background-color: var(--secondary-bg-color);
  color: var(--text-color);
  font-family:
    Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 14px;
  line-height: 1.5;
  resize: none;
}

.placeholder-container {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  color: var(--subtle-text-color);
  font-size: 1.2em;
}
</style> 
