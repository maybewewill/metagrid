import * as ipc from "$lib/ipc";
import type { Account, MetaSnapshot, Settings, Status } from "$lib/types";

export type View = "onboarding" | "dashboard" | "settings";

class Store {
  snapshot = $state<MetaSnapshot | null>(null);
  status = $state<Status>({ kind: "Idle" });
  settings = $state<Settings | null>(null);
  accounts = $state<Account[]>([]);
  view = $state<View>("onboarding");
  dashMode = $state<"list" | "preview">("list");
  loading = $state(false);
  portraitDir = $state<string | null>(null);

  isFresh = $derived.by(() => {
    if (this.status.kind !== "Ok" || !this.snapshot || !this.settings) return false;
    const fetchedAt = new Date(this.snapshot.fetched_at).getTime();
    const intervalMs = this.settings.interval_hours * 60 * 60 * 1000;
    return Date.now() - fetchedAt < intervalMs;
  });

  async init(): Promise<void> {
    this.loading = true;
    try {
      const [settings, snapshot, status, accounts, portraitDir] = await Promise.all([
        ipc.getSettings(),
        ipc.getSnapshot(),
        ipc.getStatus(),
        ipc.listAccounts(),
        ipc.getPortraitDir(),
      ]);
      this.settings = settings;
      this.snapshot = snapshot;
      this.status = status;
      this.accounts = accounts;
      this.portraitDir = portraitDir;
      this.view = settings.onboarded ? "dashboard" : "onboarding";

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

  async refresh(): Promise<void> {
    this.loading = true;
    try {
      this.snapshot = await ipc.refreshNow();
    } finally {
      this.loading = false;
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
