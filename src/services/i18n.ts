import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "@/locales/en.json";
import ru from "@/locales/ru.json";
import tw from "@/locales/zh-tw.json";
import cn from "@/locales/zh-cn.json";
import fa from "@/locales/fa.json";

const resources = {
  en: { translation: en },
  ru: { translation: ru },
  tw: { translation: tw },
  cn: { translation: cn },
  fa: { translation: fa },
};

i18n.use(initReactI18next).init({
  resources,
  lng: "en",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
  },
});
