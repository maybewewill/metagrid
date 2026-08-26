<script lang="ts">
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { store } from "$lib/store.svelte";
  import { setLanguage, type Lang } from "$lib/i18n";
  import * as ipc from "$lib/ipc";
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { Select, SelectTrigger, SelectContent, SelectItem } from "$lib/components/ui/select";
  import { Switch } from "$lib/components/ui/switch";
  import { Button } from "$lib/components/ui/button";
  import { Separator } from "$lib/components/ui/separator";
  import { ArrowLeft, Check, RefreshCw, Download, Sparkles } from "@lucide/svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import type { Settings as SettingsShape, UpdateInfo } from "$lib/types";

  const intervalOptions = [
    { value: 0.5, label: () => $_("settings.interval_30m") },
    { value: 1, label: () => $_("settings.interval_hours", { values: { n: 1 } }) },
    { value: 3, label: () => $_("settings.interval_hours", { values: { n: 3 } }) },
    { value: 6, label: () => $_("settings.interval_hours", { values: { n: 6 } }) },
    { value: 12, label: () => $_("settings.interval_hours", { values: { n: 12 } }) },
    { value: 24, label: () => $_("settings.interval_hours", { values: { n: 24 } }) },
  ];

  const DEFAULTS: SettingsShape = {
    top_n: 10,
    sort: "Pickrate",
    interval_hours: 6,
    account_id: null,
    autostart: true,
    layout_columns: true,
    lang: "en",
    onboarded: true,
    role_labels: "named",
    grid_mode: "separate",
    merge_target: null,
  };

  let local = $state<SettingsShape>({ ...DEFAULTS, ...(store.settings ?? {}) });
  let saving = $state(false);
  let checkingUpdate = $state(false);
  let updateInfo = $state<UpdateInfo | null>(null);
  let gridConfigs = $state<string[]>([]);

  onMount(async () => {
    try {
      gridConfigs = await ipc.listGridConfigs();
    } catch {
      gridConfigs = [];
    }
  });

  const accountLabel = $derived(local.account_id ?? $_("settings.all_accounts"));

  async function save() {
    saving = true;
    try {
      await store.saveSettings(local);
      try {
        await ipc.setAutostart(local.autostart);
      } catch (err) {
        console.warn("autostart toggle non-fatal:", err);
      }
      toast.success($_("settings.saved"));
      store.go("dashboard");
    } catch (e) {
      toast.error(String(e));
    } finally {
      saving = false;
    }
  }

  function onLanguageChange(l: string) {
    local.lang = l as Lang;
    setLanguage(l as Lang);
  }

  async function checkUpdates() {
    checkingUpdate = true;
    try {
      const info = await ipc.checkUpdate();
      updateInfo = info;
      if (!info.available) {
        toast.success($_("settings.up_to_date"));
      } else {
        toast.info($_("settings.update_available", { values: { version: info.latest_version } }));
      }
    } catch (err) {
      toast.error(String(err));
    } finally {
      checkingUpdate = false;
    }
  }

  async function openUpdateLink() {
    if (updateInfo?.download_url || updateInfo?.release_url) {
      await openUrl(updateInfo.download_url || updateInfo.release_url);
    }
  }
</script>

<div class="flex h-full flex-col font-sans">
  <header class="flex h-11 shrink-0 items-center gap-2 border-b border-border px-2">
    <Button
      variant="ghost"
      size="icon-sm"
      class="size-8 rounded-sm"
      aria-label={$_("settings.back")}
      onclick={() => store.go("dashboard")}
    >
      <ArrowLeft size={16} />
    </Button>
    <h1 class="text-sm font-semibold">{$_("settings.title")}</h1>
  </header>

  <div class="scroll-thin min-h-0 flex-1 overflow-auto">
    <div class="mx-auto flex max-w-xl flex-col px-4 py-2">
      <div class="flex items-center justify-between gap-4 py-3.5">
        <div class="flex min-w-0 flex-col">
          <span class="text-sm font-semibold">{$_("settings.autostart")}</span>
          <span class="text-xs text-muted-foreground">{$_("settings.autostart_hint")}</span>
        </div>
        <button
          type="button"
          role="switch"
          aria-label={$_("settings.autostart")}
          aria-checked={local.autostart}
          class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full p-0.5 border border-zinc-700 transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-1 focus-visible:ring-white {local.autostart ? 'bg-white' : 'bg-zinc-800'}"
          onclick={() => (local.autostart = !local.autostart)}
        >
          <span
            aria-hidden="true"
            class="pointer-events-none block size-[18px] transform rounded-full shadow transition-transform duration-200 ease-in-out {local.autostart ? 'translate-x-5 bg-zinc-950' : 'translate-x-0 bg-zinc-400'}"
          ></span>
        </button>
      </div>
      <Separator />

      <div class="flex items-center justify-between gap-4 py-3.5">
        <div class="flex min-w-0 flex-col">
          <span class="text-sm font-semibold">{$_("settings.role_labels")}</span>
          <span class="text-xs text-muted-foreground">
            {local.role_labels === "named" ? $_("settings.role_named_subtext") : $_("settings.role_pos_subtext")}
          </span>
        </div>
        <div class="inline-flex rounded-sm bg-zinc-900 p-0.5 border border-border">
          <button
            type="button"
            class="rounded-sm px-3 py-1.5 text-xs font-semibold transition-all {local.role_labels === 'named' ? 'bg-white text-zinc-950 font-bold shadow-sm' : 'text-zinc-400 hover:text-white'}"
            onclick={() => (local.role_labels = 'named')}
          >
            {$_("settings.role_named")}
          </button>
          <button
            type="button"
            class="rounded-sm px-3 py-1.5 text-xs font-semibold transition-all {local.role_labels === 'pos' ? 'bg-white text-zinc-950 font-bold shadow-sm' : 'text-zinc-400 hover:text-white'}"
            onclick={() => (local.role_labels = 'pos')}
          >
            {$_("settings.role_pos")}
          </button>
        </div>
      </div>
      <Separator />

      <div class="flex items-center justify-between gap-4 py-3.5">
        <div class="flex min-w-0 flex-col">
          <span class="text-sm font-semibold">{$_("settings.grid_mode")}</span>
          <span class="text-xs text-muted-foreground">
            {local.grid_mode === "merge" ? $_("settings.grid_mode_merge_hint") : $_("settings.grid_mode_separate_hint")}
          </span>
        </div>
        <div class="inline-flex rounded-sm bg-zinc-900 p-0.5 border border-border">
          <button
            type="button"
            class="rounded-sm px-3 py-1.5 text-xs font-semibold transition-all {local.grid_mode === 'separate' ? 'bg-white text-zinc-950 font-bold shadow-sm' : 'text-zinc-400 hover:text-white'}"
            onclick={() => (local.grid_mode = 'separate')}
          >
            {$_("settings.grid_mode_separate")}
          </button>
          <button
            type="button"
            class="rounded-sm px-3 py-1.5 text-xs font-semibold transition-all {local.grid_mode === 'merge' ? 'bg-white text-zinc-950 font-bold shadow-sm' : 'text-zinc-400 hover:text-white'}"
            onclick={() => (local.grid_mode = 'merge')}
          >
            {$_("settings.grid_mode_merge")}
          </button>
        </div>
      </div>

      {#if local.grid_mode === "merge"}
        <div class="flex items-center justify-between gap-4 py-3.5">
          <div class="flex min-w-0 flex-col">
            <span class="text-sm font-medium">{$_("settings.merge_target")}</span>
            <span class="text-xs text-muted-foreground">{$_("settings.merge_target_hint")}</span>
          </div>
          {#if gridConfigs.length === 0}
            <span class="text-xs text-muted-foreground">{$_("settings.merge_target_empty")}</span>
          {:else}
            <Select
              type="single"
              value={local.merge_target ?? ""}
              onValueChange={(v) => (local.merge_target = v || null)}
            >
              <SelectTrigger class="w-48 truncate rounded-sm text-xs">
                {local.merge_target ?? $_("settings.merge_target_placeholder")}
              </SelectTrigger>
              <SelectContent class="rounded-sm">
                {#each gridConfigs as name (name)}
                  <SelectItem value={name} class="rounded-sm text-xs">{name}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          {/if}
        </div>
      {/if}
      <Separator />

      <div class="flex items-center justify-between gap-4 py-3.5">
        <span class="text-sm font-medium">{$_("settings.interval")}</span>
        <Select
          type="single"
          value={String(local.interval_hours)}
          onValueChange={(v) => (local.interval_hours = Number(v))}
        >
          <SelectTrigger class="w-32 rounded-sm text-xs">
            {local.interval_hours === 0.5
              ? $_("settings.interval_30m")
              : $_("settings.interval_hours", { values: { n: local.interval_hours } })}
          </SelectTrigger>
          <SelectContent class="rounded-sm">
            {#each intervalOptions as opt (opt.value)}
              <SelectItem value={String(opt.value)} class="rounded-sm text-xs">{opt.label()}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
      <Separator />

      <div class="flex items-center justify-between gap-4 py-3.5">
        <span class="text-sm font-medium">{$_("settings.account")}</span>
        <Select
          type="single"
          value={local.account_id ?? "all"}
          onValueChange={(v) => (local.account_id = v === "all" ? null : v)}
        >
          <SelectTrigger class="w-48 truncate rounded-sm text-xs">{accountLabel}</SelectTrigger>
          <SelectContent class="rounded-sm">
            <SelectItem value="all" class="rounded-sm text-xs">{$_("settings.all_accounts")}</SelectItem>
            {#each store.accounts as a (a.id)}
              <SelectItem value={a.id} class="rounded-sm text-xs">{a.id}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
      <Separator />

      <div class="flex items-center justify-between gap-4 py-3.5">
        <span class="text-sm font-medium">{$_("settings.language")}</span>
        <div class="inline-flex rounded-sm bg-zinc-900 p-0.5 border border-border">
          <button
            type="button"
            class="rounded-sm px-4 py-1 text-xs font-semibold transition-all {local.lang === 'en' ? 'bg-white text-zinc-950 font-bold shadow-sm' : 'text-zinc-400 hover:text-white'}"
            onclick={() => onLanguageChange('en')}
          >
            EN
          </button>
          <button
            type="button"
            class="rounded-sm px-4 py-1 text-xs font-semibold transition-all {local.lang === 'ru' ? 'bg-white text-zinc-950 font-bold shadow-sm' : 'text-zinc-400 hover:text-white'}"
            onclick={() => onLanguageChange('ru')}
          >
            RU
          </button>
        </div>
      </div>
      <Separator />

      <div class="flex flex-col gap-3 py-3.5">
        <div class="flex items-center justify-between gap-4">
          <div class="flex min-w-0 flex-col">
            <span class="text-sm font-semibold">{$_("settings.updates")}</span>
            <span class="text-xs text-muted-foreground">
              {updateInfo?.current_version ?? "v0.1.1"}
            </span>
          </div>

          <Button
            variant="outline"
            size="sm"
            class="gap-1.5 rounded-sm text-xs"
            disabled={checkingUpdate}
            onclick={checkUpdates}
          >
            <RefreshCw size={13} class={checkingUpdate ? "animate-spin" : ""} />
            <span>{checkingUpdate ? $_("settings.checking_updates") : $_("settings.check_updates")}</span>
          </Button>
        </div>

        {#if updateInfo?.available}
          <div class="flex items-center justify-between gap-3 rounded-sm border border-emerald-500/30 bg-emerald-950/20 p-3 text-xs text-emerald-300">
            <div class="flex items-center gap-2">
              <Sparkles size={16} class="shrink-0 text-emerald-400" />
              <span>{$_("settings.update_available", { values: { version: updateInfo.latest_version } })}</span>
            </div>
            <Button
              size="sm"
              class="gap-1.5 rounded-sm bg-emerald-500 text-zinc-950 font-bold hover:bg-emerald-400 text-xs"
              onclick={openUpdateLink}
            >
              <Download size={13} />
              <span>{$_("settings.download_update")}</span>
            </Button>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <footer class="flex shrink-0 items-center justify-end gap-2 border-t border-border px-4 py-2.5">
    <Button variant="ghost" class="rounded-sm text-xs" onclick={() => store.go("dashboard")}>{$_("settings.back")}</Button>
    <Button class="gap-2 rounded-sm text-xs" disabled={saving} onclick={save}>
      <Check size={14} />
      {$_("settings.save")}
    </Button>
  </footer>
</div>
