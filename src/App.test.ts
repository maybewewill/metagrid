import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
vi.mock("@tauri-apps/api/core", () => ({ convertFileSrc: (p: string) => p }));
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    minimize: vi.fn(),
    hide: vi.fn(),
    toggleMaximize: vi.fn(),
    isMaximized: vi.fn(async () => false),
    onResized: vi.fn(async () => () => {}),
  }),
}));
vi.mock("motion", () => ({ animate: vi.fn() }));
vi.mock("$lib/store.svelte", () => ({
  store: {
    view: "onboarding",
    init: vi.fn(async () => {}),
    accounts: [],
    refresh: vi.fn(),
    saveSettings: vi.fn(),
    go: vi.fn(),
    snapshot: null,
    settings: null,
    loading: false,
    status: { kind: "Idle" },
  },
}));
vi.mock("$lib/ipc", () => ({
  onRefreshDone: vi.fn(async () => () => {}),
  onRefreshError: vi.fn(async () => () => {}),
}));
import App from "./App.svelte";
describe("App", () =>
  it("shows onboarding when not onboarded", () => {
    render(App);
    expect(screen.getAllByText(/MetaGrid/).length).toBeGreaterThan(0);
  }));
