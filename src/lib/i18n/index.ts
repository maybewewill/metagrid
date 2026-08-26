import { addMessages, init, locale, getLocaleFromNavigator } from "svelte-i18n";
import en from "./messages/en.json";
import ru from "./messages/ru.json";

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

export function setLanguage(l: Lang) {
  locale.set(l);
  // TODO: persist via store (task 4.4)
}
