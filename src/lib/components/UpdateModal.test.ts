import { render, screen, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";

const { mockInstallUpdate, mockOnUpdateProgress } = vi.hoisted(() => ({
  mockInstallUpdate: vi.fn(async () => {}),
  mockOnUpdateProgress: vi.fn(async () => () => {}),
}));

vi.mock("$lib/ipc", () => ({
  installUpdate: mockInstallUpdate,
  onUpdateProgress: mockOnUpdateProgress,
}));

import { store } from "$lib/store.svelte";
import UpdateModal from "./UpdateModal.svelte";

describe("UpdateModal", () => {
  it("renders when update is available and showUpdateModal is true", async () => {
    store.updateInfo = {
      available: true,
      current_version: "v1.1.2",
      latest_version: "v1.2.0",
      release_url: "https://example.com",
      release_notes: "Cool new features",
      download_url: "https://example.com/setup.exe",
    };
    store.showUpdateModal = true;

    render(UpdateModal);
    expect(screen.getByRole("dialog")).toBeDefined();
    expect(screen.getByText("Cool new features")).toBeDefined();

    const updateBtn = screen.getByRole("button", { name: /update now|обновить сейчас/i });
    await fireEvent.click(updateBtn);
    expect(mockInstallUpdate).toHaveBeenCalledWith("https://example.com/setup.exe");
  });
});
