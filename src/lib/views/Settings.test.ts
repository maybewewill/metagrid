import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
const { saveSettings, go } = vi.hoisted(() => ({ saveSettings: vi.fn(), go: vi.fn() }));
vi.mock("$lib/store.svelte", () => ({
  store: {
    settings: {
      top_n: 10,
      sort: "Pickrate",
      interval_hours: 6,
      account_id: "111",
      autostart: true,
      layout_columns: true,
      lang: "en",
      onboarded: true,
    },
    accounts: [{ id: "111" }],
    saveSettings,
    go,
  },
}));
import Settings from "$lib/views/Settings.svelte";
describe("Settings", () =>
  it("Save persists and returns to dashboard", async () => {
    render(Settings);
    await fireEvent.click(screen.getByRole("button", { name: /save/i }));
    expect(saveSettings).toHaveBeenCalled();
    expect(go).toHaveBeenCalledWith("dashboard");
  }));
