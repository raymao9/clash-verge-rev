import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "@/locales/en.json";
import tw from "@/locales/zh-tw.json";
import cn from "@/locales/zh-cn.json";

const resources = {
  en: { translation: en },
  tw: { translation: tw },
  cn: { translation: cn },
};

i18n.use(initReactI18next).init({
  resources,
  lng: "en",
  interpolation: {
    escapeValue: false,
  },
});
