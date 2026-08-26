import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
const { saveSettings, go, setAutostart } = vi.hoisted(() => ({
  saveSettings: vi.fn(),
  go: vi.fn(),
  setAutostart: vi.fn(),
}));
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
vi.mock("$lib/ipc", () => ({
  setAutostart,
  checkUpdate: vi.fn().mockResolvedValue({ available: false, current_version: "v0.1.1", latest_version: "v0.1.1", release_url: "" }),
  listGridConfigs: vi.fn().mockResolvedValue(["Main Layout", "2pos"]),
}));
vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl: vi.fn() }));
import Settings from "$lib/views/Settings.svelte";
describe("Settings", () =>
  it("Save persists and returns to dashboard", async () => {
    render(Settings);
    await fireEvent.click(screen.getByRole("button", { name: /save/i }));
    expect(saveSettings).toHaveBeenCalled();
    expect(setAutostart).toHaveBeenCalledWith(true);
    expect(go).toHaveBeenCalledWith("dashboard");
  }));
