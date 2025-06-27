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

  // Initialize settings from the backend before mounting the app
  const settingsStore = useSettingsStore();
  await settingsStore.initialize_settings();

  app.mount("#app");
}

main();
