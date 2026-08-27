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

export function fetchOnly(): Promise<MetaSnapshot> {
  return invoke("fetch_only");
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

export function getAppVersion(): Promise<string> {
  return invoke("get_app_version");
}

export function checkUpdate(): Promise<import("$lib/types").UpdateInfo> {
  return invoke("check_update");
}

export function installUpdate(downloadUrl?: string): Promise<void> {
  return invoke("install_update", { downloadUrl: downloadUrl ?? null });
}

export function onUpdateProgress(cb: (percent: number) => void): Promise<UnlistenFn> {
  return listen<number>("metagrid://update-progress", (e) => cb(e.payload));
}

export function listGridConfigs(): Promise<string[]> {
  return invoke("list_grid_configs");
}

export function getTournaments(): Promise<import("$lib/types").Tournament[]> {
  return invoke("get_tournaments");
}

export function fetchTournaments(): Promise<import("$lib/types").Tournament[]> {
  return invoke("fetch_tournaments");
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
