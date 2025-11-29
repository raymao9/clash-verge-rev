import dayjs from "dayjs";
import i18n from "i18next";
import { initReactI18next } from "react-i18next";

//
import "dayjs/locale/zh-tw";
export const FALLBACK_LANGUAGE = "tw";
//

export const supportedLanguages = ["tw", "zh", "en"];

export const languages: Record<string, any> = supportedLanguages.reduce(
  (acc, lang) => {
    acc[lang] = {};
    return acc;
  },
  {} as Record<string, any>,
);

export const loadLanguage = async (language: string) => {
  // 🚨 移除強制轉換邏輯，直接使用傳入的語言代碼
  const finalLanguage = language;
  try {
    const module = await import(`@/locales/${finalLanguage}.json`);
    return module.default;
  } catch (error) {
    console.warn(
      `Failed to load language ${language}, fallback to tw, ${error}`,
    );
    // 檢查 language 是否已經是 tw，避免無限循環
    if (language === "tw") {
      console.error("Fatal: Failed to load tw fallback file.");
      const finalFallback = await import("@/locales/en.json");
      return finalFallback.default;
    }
    // 🚨 關鍵修改：將後備檔案從 'en.json' 改為 'tw.json'
    const fallback = await import("@/locales/tw.json");
    return fallback.default;
  }
};

i18n.use(initReactI18next).init({
  resources: {},
  lng: FALLBACK_LANGUAGE,
  fallbackLng: FALLBACK_LANGUAGE,
  load: "all",
  interpolation: {
    escapeValue: false,
  },
});

export const changeLanguage = async (language: string) => {
  if (!i18n.hasResourceBundle(language, "translation")) {
    const resources = await loadLanguage(language);
    i18n.addResourceBundle(language, "translation", resources);
  }

  await i18n.changeLanguage(language);
};

// 修改 initializeLanguage 函數
export const initializeLanguage = async (initialLanguage: string = "tw") => {
  // 1. 設置 i18next 語言
  await changeLanguage(initialLanguage);
  // 2. 設置 Day.js 語言
  if (initialLanguage === "tw") {
    dayjs.locale("zh-tw");
  } else if (initialLanguage === "zh") {
    dayjs.locale("zh-cn");
  } else {
    dayjs.locale("en");
  }
};
