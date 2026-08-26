import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
vi.mock("@tauri-apps/api/core", () => ({ convertFileSrc: (p: string) => p }));
vi.mock("motion", () => ({ animate: vi.fn() }));
vi.mock("$lib/store.svelte", () => ({
  store: {
    loading: false,
    status: { kind: "Ok" },
    refresh: vi.fn(),
    go: vi.fn(),
    snapshot: {
      patch: "7.41e",
      fetched_at: new Date().toISOString(),
      source: "d2pt",
      roles: [
        {
          position: "Pos1",
          role_winrate: 0.5,
          heroes: [
            { hero_id: 1, name: "AM", slug: "antimage", winrate: 0.5, pickrate: 0.1, matches: 1 },
          ],
        },
      ],
    },
  },
}));
import Dashboard from "$lib/views/Dashboard.svelte";
describe("Dashboard", () =>
  it("renders role columns from snapshot", () => {
    render(Dashboard);
    expect(screen.getByText("POS 1 — Carry")).toBeInTheDocument();
  }));
