import "@testing-library/jest-dom/vitest";
import { init, addMessages } from "svelte-i18n";
import en from "$lib/i18n/messages/en.json";

addMessages("en", en);
init({ fallbackLocale: "en", initialLocale: "en" });

if (typeof globalThis.ResizeObserver === "undefined") {
  globalThis.ResizeObserver = class ResizeObserver {
    observe() {}
    unobserve() {}
    disconnect() {}
  };
}

if (typeof globalThis.matchMedia === "undefined") {
  globalThis.matchMedia = ((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener() {},
    removeListener() {},
    addEventListener() {},
    removeEventListener() {},
    dispatchEvent() {
      return false;
    },
  })) as unknown as typeof globalThis.matchMedia;
}
