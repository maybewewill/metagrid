<script lang="ts">
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
  import { ArrowLeft, Check } from "@lucide/svelte";
  import type { Settings as SettingsShape } from "$lib/types";

  const intervalOptions = [1, 3, 6, 12, 24];

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
  };

  // Edit a local copy; nothing is applied until Save.
  let local = $state<SettingsShape>({ ...DEFAULTS, ...(store.settings ?? {}) });
  let saving = $state(false);

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
    setLanguage(l as Lang); // live preview
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
      <!-- Start with Windows (Autostart) -->
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

      <!-- Role labels -->
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

      <!-- Refresh interval -->
      <div class="flex items-center justify-between gap-4 py-3.5">
        <span class="text-sm font-medium">{$_("settings.interval")}</span>
        <Select
          type="single"
          value={String(local.interval_hours)}
          onValueChange={(v) => (local.interval_hours = Number(v))}
        >
          <SelectTrigger class="w-32 rounded-sm text-xs">{$_("settings.interval_hours", { values: { n: local.interval_hours } })}</SelectTrigger>
          <SelectContent class="rounded-sm">
            {#each intervalOptions as h (h)}
              <SelectItem value={String(h)} class="rounded-sm text-xs">{$_("settings.interval_hours", { values: { n: h } })}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
      <Separator />

      <!-- Account -->
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

      <!-- Language -->
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
