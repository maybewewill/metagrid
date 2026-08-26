import { render, screen } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
vi.mock("@tauri-apps/api/core", () => ({ convertFileSrc: (p: string) => "asset://" + p }));
vi.mock("motion", () => ({ animate: vi.fn() }));
import HeroCard from "$lib/components/HeroCard.svelte";
describe("HeroCard", () => {
  it("shows name and winrate", () => {
    render(HeroCard, {
      hero: {
        hero_id: 1,
        name: "Anti-Mage",
        slug: "antimage",
        winrate: 0.53,
        pickrate: 0.12,
        matches: 900,
        d2pt_rating: 3200,
      },
      rank: 1,
    });
    expect(screen.getByText("Anti-Mage")).toBeInTheDocument();
    expect(screen.getByText("53%")).toBeInTheDocument();
    expect(screen.getByText("3200")).toBeInTheDocument();
  });
});
