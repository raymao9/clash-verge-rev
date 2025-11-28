import i18n from "i18next";
import { initReactI18next } from "react-i18next";

export const supportedLanguages = ["zhtw", "zh", "en"];

export const languages: Record<string, any> = supportedLanguages.reduce(
  (acc, lang) => {
    acc[lang] = {};
    return acc;
  },
  {} as Record<string, any>,
);

export const loadLanguage = async (language: string) => {
  try {
    const module = await import(`@/locales/${language}.json`);
    return module.default;
  } catch (error) {
    console.warn(
      `Failed to load language ${language}, fallback to en, ${error}`,
    );
    const fallback = await import("@/locales/en.json");
    return fallback.default;
  }
};

i18n.use(initReactI18next).init({
  resources: {},
  lng: "zhtw",
  fallbackLng: "zhtw",
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

export const initializeLanguage = async (initialLanguage: string = "zh") => {
  await changeLanguage(initialLanguage);
};
