import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
vi.mock("@tauri-apps/api/core", () => ({ convertFileSrc: (p: string) => p }));
vi.mock("motion", () => ({ animate: vi.fn() }));
import RoleColumn from "$lib/components/RoleColumn.svelte";
const role = {
  position: "Pos1",
  role_winrate: 0.52,
  heroes: [
    { hero_id: 1, name: "AM", slug: "antimage", winrate: 0.53, pickrate: 0.12, matches: 1 },
    {
      hero_id: 2,
      name: "PA",
      slug: "phantom_assassin",
      winrate: 0.51,
      pickrate: 0.1,
      matches: 1,
    },
  ],
};
describe("RoleColumn", () => {
  it("renders label and all heroes", () => {
    render(RoleColumn, { role });
    expect(screen.getByText("POS 1 — Carry")).toBeInTheDocument();
    expect(screen.getAllByRole("img").length).toBe(2);
  });
});
