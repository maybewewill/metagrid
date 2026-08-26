import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
const { refresh, fetchOnly, saveSettings, go } = vi.hoisted(() => ({
  refresh: vi.fn(async () => {}),
  fetchOnly: vi.fn(async () => {}),
  saveSettings: vi.fn(async () => {}),
  go: vi.fn(),
}));
vi.mock("$lib/store.svelte", () => ({ store: { accounts: [{ id: "111" }], refresh, fetchOnly, saveSettings, go } }));
vi.mock("$lib/ipc", () => ({ listGridConfigs: vi.fn(async () => ["Main Layout"]) }));
import Onboarding from "$lib/views/Onboarding.svelte";

describe("Onboarding", () =>
  it("fetches first meta then finish saves onboarded + goes dashboard", async () => {
    render(Onboarding);
    await fireEvent.click(screen.getByRole("button", { name: /fetch meta/i }));
    expect(fetchOnly).toHaveBeenCalled();

    const finishBtn = await screen.findByRole("button", { name: /get started|finish|done/i });
    await fireEvent.click(finishBtn);
    expect(saveSettings).toHaveBeenCalledWith(expect.objectContaining({ onboarded: true }));
    expect(go).toHaveBeenCalledWith("dashboard");
  }));
