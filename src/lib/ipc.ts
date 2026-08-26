import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { Account, MetaSnapshot, Settings, Status } from "$lib/types";

export const EVENTS = {
  REFRESH_STARTED: "metagrid://refresh-started",
  REFRESH_DONE: "metagrid://refresh-done",
  REFRESH_ERROR: "metagrid://refresh-error",
  STATUS: "metagrid://status",
} as const;

export function getSnapshot(): Promise<MetaSnapshot | null> {
  return invoke("get_snapshot");
}

export function getStatus(): Promise<Status> {
  return invoke("get_status");
}

export function getSettings(): Promise<Settings> {
  return invoke("get_settings");
}

export function saveSettings(s: Settings): Promise<void> {
  return invoke("save_settings", { new: s });
}

export function listAccounts(): Promise<Account[]> {
  return invoke("list_accounts");
}

export function refreshNow(): Promise<MetaSnapshot> {
  return invoke("refresh_now");
}

export function getAutostart(): Promise<boolean> {
  return invoke("get_autostart");
}

export function setAutostart(enabled: boolean): Promise<void> {
  return invoke("set_autostart", { enabled });
}

export function getPortraitDir(): Promise<string> {
  return invoke("get_portrait_dir");
}

export function checkUpdate(): Promise<import("$lib/types").UpdateInfo> {
  return invoke("check_update");
}

export function onRefreshDone(cb: (payload: MetaSnapshot) => void): Promise<UnlistenFn> {
  return listen<MetaSnapshot>(EVENTS.REFRESH_DONE, (e) => cb(e.payload));
}

export function onRefreshError(cb: (payload: string) => void): Promise<UnlistenFn> {
  return listen<string>(EVENTS.REFRESH_ERROR, (e) => cb(e.payload));
}

export function onStatus(cb: (payload: Status) => void): Promise<UnlistenFn> {
  return listen<Status>(EVENTS.STATUS, (e) => cb(e.payload));
}
