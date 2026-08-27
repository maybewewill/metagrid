import * as ipc from "$lib/ipc";
import type { Account, MetaSnapshot, Settings, Status, Tournament } from "$lib/types";

export type View = "onboarding" | "dashboard" | "settings";

class Store {
  snapshot = $state<MetaSnapshot | null>(null);
  status = $state<Status>({ kind: "Idle" });
  settings = $state<Settings | null>(null);
  accounts = $state<Account[]>([]);
  tournaments = $state<Tournament[]>([]);
  appVersion = $state<string>("v1.4.1");
  view = $state<View>("onboarding");
  loading = $state(false);
  fetchingOnly = $state(false);
  portraitDir = $state<string | null>(null);
  updateInfo = $state<import("$lib/types").UpdateInfo | null>(null);
  showUpdateModal = $state<boolean>(false);

  isFresh = $derived.by(() => {
    if (this.status.kind !== "Ok" || !this.snapshot || !this.settings) return false;
    const fetchedAt = new Date(this.snapshot.fetched_at).getTime();
    const intervalMs = this.settings.interval_hours * 60 * 60 * 1000;
    return Date.now() - fetchedAt < intervalMs;
  });

  async init(): Promise<void> {
    this.loading = true;
    try {
      const [settings, snapshot, status, accounts, portraitDir, tournaments, appVersion] = await Promise.all([
        ipc.getSettings(),
        ipc.getSnapshot(),
        ipc.getStatus(),
        ipc.listAccounts(),
        ipc.getPortraitDir(),
        ipc.getTournaments().catch(() => []),
        ipc.getAppVersion().catch(() => "v1.2.3"),
      ]);
      this.settings = settings;
      this.snapshot = snapshot;
      this.status = status;
      this.accounts = accounts;
      this.portraitDir = portraitDir;
      this.tournaments = tournaments;
      this.appVersion = appVersion;
      this.view = settings.onboarded ? "dashboard" : "onboarding";

      this.checkForUpdates(true).catch(() => {});

      await ipc.onRefreshDone((snap) => {
        this.snapshot = snap;
        this.status = { kind: "Ok" };
      });
      await ipc.onRefreshError((detail) => {
        this.status = { kind: "Error", detail };
      });
      await ipc.onStatus((s) => {
        this.status = s;
      });
    } finally {
      this.loading = false;
    }
  }

  async checkForUpdates(silent: boolean = false): Promise<import("$lib/types").UpdateInfo | null> {
    try {
      const info = await ipc.checkUpdate();
      this.updateInfo = info;
      if (info.available) {
        this.showUpdateModal = true;
      }
      return info;
    } catch (e) {
      if (!silent) throw e;
      return null;
    }
  }

  async refresh(): Promise<void> {
    this.loading = true;
    try {
      this.snapshot = await ipc.refreshNow();
    } finally {
      this.loading = false;
    }
  }

  async fetchOnly(): Promise<MetaSnapshot> {
    this.loading = true;
    this.fetchingOnly = true;
    try {
      const snap = await ipc.fetchOnly();
      this.snapshot = snap;
      this.status = { kind: "Ok" };
      return snap;
    } catch (e) {
      this.status = { kind: "Error", detail: String(e) };
      throw e;
    } finally {
      this.loading = false;
      this.fetchingOnly = false;
    }
  }

  async fetchTournaments(): Promise<Tournament[]> {
    try {
      const list = await ipc.fetchTournaments();
      if (list && list.length > 0) {
        this.tournaments = list;
      }
      return this.tournaments;
    } catch {
      return this.tournaments;
    }
  }

  async saveSettings(patch: Partial<Settings>): Promise<void> {
    if (!this.settings) return;
    const merged: Settings = { ...this.settings, ...patch };
    await ipc.saveSettings(merged);
    this.settings = merged;
  }

  go(view: View): void {
    this.view = view;
  }
}

export const store = new Store();
