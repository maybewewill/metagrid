import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
// vi.mock factories are hoisted above top-level const declarations, so the
// mocked fn is created via vi.hoisted.
const { refresh, go } = vi.hoisted(() => ({ refresh: vi.fn(), go: vi.fn() }));
vi.mock("$lib/store.svelte", () => ({
  store: {
    snapshot: { patch: "7.41e", fetched_at: new Date().toISOString(), source: "d2pt", roles: [] },
    status: { kind: "Ok" },
    refresh,
    go,
  },
}));
import StatusBar from "$lib/components/StatusBar.svelte";

describe("StatusBar", () => {
  it("refresh button triggers store.refresh", async () => {
    render(StatusBar);
    await fireEvent.click(screen.getByRole("button", { name: /refresh/i }));
    expect(refresh).toHaveBeenCalled();
  });

  it("gear button navigates to settings", async () => {
    render(StatusBar);
    const buttons = screen.getAllByRole("button");
    // the settings gear is the trailing icon button
    await fireEvent.click(buttons[buttons.length - 1]);
    expect(go).toHaveBeenCalledWith("settings");
  });
});
