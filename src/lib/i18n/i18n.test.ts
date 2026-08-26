import { describe, it, expect, beforeAll } from "vitest";
import { _ } from "svelte-i18n";
import { get } from "svelte/store";
import { setupI18n, setLanguage } from "$lib/i18n";

describe("i18n", () => {
  beforeAll(async () => {
    setupI18n("en");
  });

  it("resolves the same key in both locales", async () => {
    expect(get(_)("settings.title")).toBe("Settings");
    await setLanguage("ru");
    expect(get(_)("settings.title")).toBe("Настройки");
  });
});
