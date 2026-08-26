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
      // Autostart is an OS-level toggle — persisting the flag isn't enough,
      // we have to actually (de)register it.
      await ipc.setAutostart(local.autostart);
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
          <span class="text-sm font-medium">{$_("settings.autostart")}</span>
          <span class="text-xs text-muted-foreground">{$_("settings.autostart_hint")}</span>
        </div>
        <Switch checked={local.autostart} onCheckedChange={(v) => (local.autostart = v)} />
      </div>
      <Separator />

      <!-- Role labels -->
      <div class="flex items-center justify-between gap-4 py-3.5">
        <span class="text-sm font-medium">{$_("settings.role_labels")}</span>
        <Tabs value={local.role_labels} onValueChange={(v) => (local.role_labels = v as "named" | "pos")}>
          <TabsList class="h-8 rounded-sm bg-muted/60 p-0.5">
            <TabsTrigger value="named" class="rounded-sm px-3 text-xs">{$_("settings.role_named")}</TabsTrigger>
            <TabsTrigger value="pos" class="rounded-sm px-3 text-xs">{$_("settings.role_pos")}</TabsTrigger>
          </TabsList>
        </Tabs>
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
          <SelectTrigger class="w-32 rounded-sm text-xs">{local.interval_hours} {local.lang === 'ru' ? 'ч' : 'h'}</SelectTrigger>
          <SelectContent class="rounded-sm">
            {#each intervalOptions as h (h)}
              <SelectItem value={String(h)} class="rounded-sm text-xs">{h} {local.lang === 'ru' ? 'ч' : 'h'}</SelectItem>
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
        <Tabs value={local.lang} onValueChange={onLanguageChange}>
          <TabsList class="h-8 rounded-sm bg-muted/60 p-0.5">
            <TabsTrigger value="en" class="rounded-sm px-4 text-xs font-semibold">EN</TabsTrigger>
            <TabsTrigger value="ru" class="rounded-sm px-4 text-xs font-semibold">RU</TabsTrigger>
          </TabsList>
        </Tabs>
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
