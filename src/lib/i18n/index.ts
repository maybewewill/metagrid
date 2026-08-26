import { addMessages, init, locale, getLocaleFromNavigator } from "svelte-i18n";
import en from "./messages/en.json";
import ru from "./messages/ru.json";
import { store } from "$lib/store.svelte";

export type Lang = "en" | "ru";

// Dictionaries are small and bundled at build time, so we register them
// synchronously via addMessages() rather than svelte-i18n's async register()
// loader queue. This keeps locale switches (and setupI18n) synchronous —
// no locale-loading race for callers/tests that don't await message loads.
addMessages("en", en);
addMessages("ru", ru);

export function getInitialLang(): Lang {
  const navigatorLocale = getLocaleFromNavigator() ?? "en";
  const reduced = navigatorLocale.slice(0, 2).toLowerCase();
  return reduced === "ru" ? "ru" : "en";
}

export function setupI18n(initialLocale: Lang = getInitialLang()) {
  init({
    fallbackLocale: "en",
    initialLocale,
  });
}

export async function setLanguage(l: Lang) {
  locale.set(l);
  // Persist the choice. Guarded: store.saveSettings() is a no-op until
  // store.init() has loaded settings, so this is safe to call before
  // (or without) the store ever being initialized (e.g. in tests).
  if (store.settings) {
    await store.saveSettings({ lang: l });
  }
}
