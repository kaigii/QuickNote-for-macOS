import i18next from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import en from "./locales/en.json";
import zhTW from "./locales/zh-TW.json";

i18next.use(LanguageDetector).init({
  debug: false,
  detection: {
    order: ["navigator", "htmlTag"],
    caches: [],
  },
  resources: {
    en: {
      translation: en,
    },
    "zh-TW": {
      translation: zhTW,
    },
  },
  fallbackLng: "en",
  interpolation: {
    escapeValue: false, // Vue already escapes values
  },
});

export default i18next;
