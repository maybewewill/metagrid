import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";

const { refresh, go } = vi.hoisted(() => ({ refresh: vi.fn(), go: vi.fn() }));
vi.mock("$lib/store.svelte", () => ({
  store: {
    snapshot: { patch: "7.41e", fetched_at: new Date().toISOString(), source: "d2pt", roles: [] },
    status: { kind: "Ok" },
    refresh,
    go,
  },
}));
import { setupI18n } from "$lib/i18n";
import StatusBar from "$lib/components/StatusBar.svelte";

describe("StatusBar", () => {
  setupI18n("en");

  it("refresh button triggers store.refresh", async () => {
    render(StatusBar);
    await fireEvent.click(screen.getByRole("button", { name: /fetch & patch|refresh/i }));
    expect(refresh).toHaveBeenCalled();
  });

  it("gear button navigates to settings", async () => {
    render(StatusBar);
    const buttons = screen.getAllByRole("button");
    await fireEvent.click(buttons[buttons.length - 1]);
    expect(go).toHaveBeenCalledWith("settings");
  });
});
