import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
// vi.mock factories are hoisted above top-level const declarations, so the
// mocked fn must be created via vi.hoisted (the brief's plain `const refresh
// = vi.fn()` throws "Cannot access 'refresh' before initialization" under
// vitest's hoisting) — fixed here to the working API.
const { refresh } = vi.hoisted(() => ({ refresh: vi.fn() }));
vi.mock("$lib/store.svelte", () => ({
  store: {
    snapshot: { patch: "7.41e", fetched_at: new Date().toISOString(), source: "d2pt", roles: [] },
    status: { kind: "Ok" },
    refresh,
    go: vi.fn(),
  },
}));
const { launchDota } = vi.hoisted(() => ({ launchDota: vi.fn() }));
vi.mock("$lib/ipc", () => ({ launchDota }));
import StatusBar from "$lib/components/StatusBar.svelte";
describe("StatusBar", () => {
  it("refresh button triggers store.refresh", async () => {
    render(StatusBar);
    await fireEvent.click(screen.getByRole("button", { name: /refresh/i }));
    expect(refresh).toHaveBeenCalled();
  });

  it("play button triggers ipc.launchDota", async () => {
    render(StatusBar);
    await fireEvent.click(screen.getByRole("button", { name: /play/i }));
    expect(launchDota).toHaveBeenCalled();
  });
});
