import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
const hide = vi.fn(),
  minimize = vi.fn(),
  toggleMaximize = vi.fn(),
  isMaximized = vi.fn(async () => false),
  onResized = vi.fn(async () => () => {});
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ hide, minimize, toggleMaximize, isMaximized, onResized }),
}));
import Titlebar from "$lib/components/Titlebar.svelte";
describe("Titlebar", () => {
  it("close button hides window", async () => {
    render(Titlebar, { patch: "7.41e" });
    await fireEvent.click(screen.getByLabelText("Close"));
    expect(hide).toHaveBeenCalled();
  });

  it("minimize button hides window to tray", async () => {
    render(Titlebar, { patch: "7.41e" });
    await fireEvent.click(screen.getByLabelText("Minimize"));
    expect(hide).toHaveBeenCalled();
  });
});
