import "@testing-library/jest-dom/vitest";
import { init, addMessages } from "svelte-i18n";
import en from "$lib/i18n/messages/en.json";

// Intentionally NOT `import { setupI18n } from "$lib/i18n"` here: that
// module's index also imports `$lib/store.svelte` (for setLanguage), which
// transitively imports `$lib/ipc` and eagerly resolves the real
// "@tauri-apps/api/core"/"event" modules during setupFiles — before
// per-test-file `vi.mock(...)` calls for those specifiers take effect. That
// caches ipc.ts (and its `invoke` binding) against the real, unmocked Tauri
// bridge, which then breaks ipc.test.ts and store.svelte.test.ts (invoke()
// throws because `window.__TAURI_INTERNALS__` is undefined in jsdom).
// Registering the "en" dictionary and initializing svelte-i18n directly here
// gives every test's `$_(...)` real resolved strings without that import
// chain.
addMessages("en", en);
init({ fallbackLocale: "en", initialLocale: "en" });

// jsdom doesn't implement ResizeObserver, but bits-ui primitives (Slider,
// Select, ...) construct one unconditionally when mounted. Stub it globally
// so any component test that renders those primitives doesn't need its own
// per-file shim.
if (typeof globalThis.ResizeObserver === "undefined") {
  globalThis.ResizeObserver = class ResizeObserver {
    observe() {}
    unobserve() {}
    disconnect() {}
  };
}
