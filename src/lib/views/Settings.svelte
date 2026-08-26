<script lang="ts">
  import { _ } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { store } from "$lib/store.svelte";
  import { setLanguage, type Lang } from "$lib/i18n";
  import * as ipc from "$lib/ipc";
  import { Slider } from "$lib/components/ui/slider";
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { Select, SelectTrigger, SelectContent, SelectItem } from "$lib/components/ui/select";
  import { Switch } from "$lib/components/ui/switch";
  import { Button } from "$lib/components/ui/button";
  import { Separator } from "$lib/components/ui/separator";
  import { ArrowLeft, Check } from "@lucide/svelte";
  import type { Settings as SettingsShape, SortMetric } from "$lib/types";

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

<div class="flex h-full flex-col">
  <header class="flex h-11 shrink-0 items-center gap-2 border-b border-border px-2">
    <Button
      variant="ghost"
      size="icon-sm"
      class="size-8"
      aria-label={$_("settings.back")}
      onclick={() => store.go("dashboard")}
    >
      <ArrowLeft size={16} />
    </Button>
    <h1 class="text-sm font-semibold">{$_("settings.title")}</h1>
  </header>

  <div class="scroll-thin min-h-0 flex-1 overflow-auto">
    <div class="mx-auto flex max-w-xl flex-col px-4 py-3">
      <!-- Heroes per role -->
      <div class="flex flex-col gap-3 py-4">
        <div class="flex items-center justify-between">
          <span class="text-sm font-medium">{$_("settings.heroes_per_role")}</span>
          <span class="font-mono text-sm tabular-nums text-muted-foreground">{local.top_n}</span>
        </div>
        <Slider
          type="single"
          min={5}
          max={15}
          step={1}
          value={local.top_n}
          onValueChange={(v) => (local.top_n = v)}
        />
      </div>
      <Separator />

      <!-- Sort metric -->
      <div class="flex items-center justify-between gap-4 py-4">
        <span class="text-sm font-medium">{$_("settings.sort")}</span>
        <Tabs value={local.sort} onValueChange={(v) => (local.sort = v as SortMetric)}>
          <TabsList class="h-8">
            <TabsTrigger value="Pickrate" class="text-xs">{$_("settings.pickrate")}</TabsTrigger>
            <TabsTrigger value="Winrate" class="text-xs">{$_("settings.winrate")}</TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
      <Separator />

      <!-- Role labels -->
      <div class="flex items-center justify-between gap-4 py-4">
        <span class="text-sm font-medium">{$_("settings.role_labels")}</span>
        <Tabs value={local.role_labels} onValueChange={(v) => (local.role_labels = v as "named" | "pos")}>
          <TabsList class="h-8">
            <TabsTrigger value="named" class="text-xs">{$_("settings.role_named")}</TabsTrigger>
            <TabsTrigger value="pos" class="text-xs">{$_("settings.role_pos")}</TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
      <Separator />

      <!-- Refresh interval -->
      <div class="flex items-center justify-between gap-4 py-4">
        <span class="text-sm font-medium">{$_("settings.interval")}</span>
        <Select
          type="single"
          value={String(local.interval_hours)}
          onValueChange={(v) => (local.interval_hours = Number(v))}
        >
          <SelectTrigger class="w-28">{local.interval_hours} h</SelectTrigger>
          <SelectContent>
            {#each intervalOptions as h (h)}
              <SelectItem value={String(h)}>{h} h</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
      <Separator />

      <!-- Account -->
      <div class="flex items-center justify-between gap-4 py-4">
        <span class="text-sm font-medium">{$_("settings.account")}</span>
        <Select
          type="single"
          value={local.account_id ?? "all"}
          onValueChange={(v) => (local.account_id = v === "all" ? null : v)}
        >
          <SelectTrigger class="w-44 truncate">{accountLabel}</SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{$_("settings.all_accounts")}</SelectItem>
            {#each store.accounts as a (a.id)}
              <SelectItem value={a.id}>{a.id}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
      <Separator />

      <!-- Language -->
      <div class="flex items-center justify-between gap-4 py-4">
        <span class="text-sm font-medium">{$_("settings.language")}</span>
        <Tabs value={local.lang} onValueChange={onLanguageChange}>
          <TabsList class="h-8">
            <TabsTrigger value="en" class="text-xs">EN</TabsTrigger>
            <TabsTrigger value="ru" class="text-xs">RU</TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
      <Separator />

      <!-- Autostart -->
      <div class="flex items-center justify-between gap-4 py-4">
        <div class="flex min-w-0 flex-col">
          <span class="text-sm font-medium">{$_("settings.autostart")}</span>
          <span class="text-xs text-muted-foreground">{$_("settings.autostart_hint")}</span>
        </div>
        <Switch checked={local.autostart} onCheckedChange={(v) => (local.autostart = v)} />
      </div>
      <Separator />

      <!-- Layout mode -->
      <div class="flex items-center justify-between gap-4 py-4">
        <div class="flex min-w-0 flex-col">
          <span class="text-sm font-medium">{$_("settings.layout")}</span>
          <span class="text-xs text-muted-foreground">{$_("settings.layout_hint")}</span>
        </div>
        <Switch
          checked={local.layout_columns}
          onCheckedChange={(v) => (local.layout_columns = v)}
        />
      </div>
    </div>
  </div>

  <footer class="flex shrink-0 items-center justify-end gap-2 border-t border-border px-4 py-3">
    <Button variant="ghost" onclick={() => store.go("dashboard")}>{$_("settings.back")}</Button>
    <Button class="gap-2" disabled={saving} onclick={save}>
      <Check size={15} />
      {$_("settings.save")}
    </Button>
  </footer>
</div>
