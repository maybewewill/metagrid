import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
const hide = vi.fn(),
  minimize = vi.fn();
vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: () => ({ hide, minimize }) }));
import Titlebar from "$lib/components/Titlebar.svelte";
describe("Titlebar", () => {
  it("close button hides window", async () => {
    render(Titlebar, { patch: "7.41e" });
    await fireEvent.click(screen.getByLabelText("Close"));
    expect(hide).toHaveBeenCalled();
  });
});
