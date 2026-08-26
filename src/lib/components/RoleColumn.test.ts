import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
vi.mock("motion", () => ({ animate: vi.fn() }));
import RoleColumn from "$lib/components/RoleColumn.svelte";
import type { RoleMeta } from "$lib/types";

const role: RoleMeta = {
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
  it("renders the role and all heroes", () => {
    render(RoleColumn, { role });
    // The "POS 1 — " prefix is stripped in the header (a numbered chip carries
    // the position), so the role name renders on its own.
    expect(screen.getByText("Carry")).toBeInTheDocument();
    // Hero portraits load from the Steam CDN — one <img> per hero.
    expect(screen.getAllByRole("img").length).toBe(2);
  });
});
