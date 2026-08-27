import { addMessages, init, locale, getLocaleFromNavigator } from "svelte-i18n";
import { store } from "$lib/store.svelte";
import { en, ru } from "./dict";

export type Lang = "en" | "ru";

export { en, ru };

addMessages("en", en);
addMessages("ru", ru);

export function getInitialLang(): Lang {
  return "en";
}

export function setupI18n(initialLocale: Lang = getInitialLang()) {
  init({
    fallbackLocale: "en",
    initialLocale,
  });
}

export async function setLanguage(l: Lang) {
  locale.set(l);
  if (store.settings) {
    await store.saveSettings({ lang: l });
  }
}
