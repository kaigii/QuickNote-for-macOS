import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./assets/styles.css";
import { useSettingsStore } from "./stores/settings";
import I18nextVue from "i18next-vue";
import i18next from "./i18n";

async function main() {
  const app = createApp(App);
  const pinia = createPinia();
  app.use(pinia);
  app.use(I18nextVue, { i18next });

  // Check if we're running in a web environment
  const isWeb = typeof window !== 'undefined' && !window.__TAURI__;
  
  if (!isWeb) {
    // Initialize settings from the backend before mounting the app (Tauri only)
  const settingsStore = useSettingsStore();
  await settingsStore.initialize_settings();
  }

  app.mount("#app");
}

main();
