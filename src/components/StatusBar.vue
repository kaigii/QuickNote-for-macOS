<script setup lang="ts">
import { computed } from "vue";
import { useTabsStore } from "../stores/tabs";
import { useTranslation } from "i18next-vue";

const { t } = useTranslation();
const tabsStore = useTabsStore();

const activeTab = computed(() =>
  tabsStore.tabs.find((tab) => tab.id === tabsStore.activeTabId),
);

const cursorPosition = computed(() => {
  if (!activeTab.value || activeTab.value.cursorPos === null) {
    return "";
  }
  const content = activeTab.value.content;
  const pos = activeTab.value.cursorPos;
  const textBeforeCursor = content.slice(0, pos);
  const line = textBeforeCursor.split("\n").length;
  const col = (textBeforeCursor.split("\n").pop()?.length ?? 0) + 1;
  return t("status_bar.cursor_pos", { line, col });
});

const fileFormat = computed(() => {
  if (!activeTab.value || !activeTab.value.path) {
    return "Plain Text";
  }
  const extension = activeTab.value.path.split(".").pop()?.toLowerCase();
  if (!extension) return "Plain Text";

  const formatMap: { [key: string]: string } = {
    md: "Markdown",
    js: "JavaScript",
    ts: "TypeScript",
    json: "JSON",
    html: "HTML",
    css: "CSS",
    rs: "Rust",
    vue: "Vue",
  };
  return formatMap[extension] || "Plain Text";
});
</script>

<template>
  <div class="status-bar">
    <div class="status-group left">
      <div class="status-item">{{ cursorPosition }}</div>
    </div>
    <div class="status-group center">
      <span>QuickNote</span>
    </div>
    <div class="status-group right">
      <div class="status-item">
        {{ t("status_bar.characters") }}:
        {{ activeTab ? activeTab.content.length : 0 }}
      </div>
      <div class="status-item">{{ fileFormat }}</div>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 10px;
  height: 25px;
  background-color: var(--bg-color);
  color: var(--text-color);
  font-size: 12px;
  border-top: 1px solid var(--border-color);
}

.status-group {
  display: flex;
  align-items: center;
  gap: 15px;
  flex: 1;
}

.status-group.left {
  justify-content: flex-start;
}

.status-group.center {
  justify-content: center;
  text-align: center;
}

.status-group.right {
  justify-content: flex-end;
}

.status-item {
  white-space: nowrap;
}
</style>
