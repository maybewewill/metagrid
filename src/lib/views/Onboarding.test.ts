import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
const { refresh, saveSettings, go } = vi.hoisted(() => ({
  refresh: vi.fn(async () => {}),
  saveSettings: vi.fn(async () => {}),
  go: vi.fn(),
}));
vi.mock("$lib/store.svelte", () => ({ store: { accounts: [{ id: "111" }], refresh, saveSettings, go } }));
import Onboarding from "$lib/views/Onboarding.svelte";
describe("Onboarding", () =>
  it("finish saves onboarded + goes dashboard", async () => {
    render(Onboarding);
    await fireEvent.click(screen.getByRole("button", { name: /get started|finish|done/i }));
    expect(saveSettings).toHaveBeenCalledWith(expect.objectContaining({ onboarded: true }));
    expect(go).toHaveBeenCalledWith("dashboard");
  }));
