import { describe, it, expect, vi, beforeEach } from "vitest";

vi.mock("$lib/ipc", () => ({
  getSettings: vi.fn(async () => ({
    top_n: 10,
    sort: "Pickrate",
    interval_hours: 6,
    account_id: null,
    autostart: true,
    layout_columns: true,
    onboarded: true,
  })),
  getSnapshot: vi.fn(async () => ({ patch: "7.41e", fetched_at: "t", source: "d2pt", roles: [] })),
  getStatus: vi.fn(async () => ({ kind: "Ok" })),
  listAccounts: vi.fn(async () => [{ id: "111" }]),
  getPortraitDir: vi.fn(async () => "C:/data/portraits"),
  refreshNow: vi.fn(async () => ({ patch: "7.42", fetched_at: "t2", source: "d2pt", roles: [] })),
  onRefreshDone: vi.fn(() => () => {}),
  onRefreshError: vi.fn(() => () => {}),
  onStatus: vi.fn(() => () => {}),
}));

import { store } from "$lib/store.svelte";

beforeEach(() => {
  store.snapshot = null;
});

describe("store", () => {
  it("init loads settings/snapshot and picks dashboard when onboarded", async () => {
    await store.init();
    expect(store.settings?.top_n).toBe(10);
    expect(store.view).toBe("dashboard");
    expect(store.portraitDir).toBe("C:/data/portraits");
  });

  it("refresh updates snapshot", async () => {
    await store.refresh();
    expect(store.snapshot?.patch).toBe("7.42");
  });
});
