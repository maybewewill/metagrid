import { describe, it, expect, vi, beforeEach } from "vitest";

const invoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...a: unknown[]) => invoke(...a) }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn(async () => () => {}) }));

import { getSnapshot, saveSettings } from "$lib/ipc";

beforeEach(() => invoke.mockReset());

describe("ipc", () => {
  it("getSnapshot calls invoke('get_snapshot')", async () => {
    invoke.mockResolvedValue(null);
    await getSnapshot();
    expect(invoke).toHaveBeenCalledWith("get_snapshot");
  });
  it("saveSettings passes settings arg", async () => {
    invoke.mockResolvedValue(undefined);
    await saveSettings({ top_n: 10 } as never);
    expect(invoke).toHaveBeenCalledWith("save_settings", { new: { top_n: 10 } });
  });
});
