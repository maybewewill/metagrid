<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { setLanguage, type Lang } from "$lib/i18n";
  import { Slider } from "$lib/components/ui/slider";
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { Select, SelectTrigger, SelectContent, SelectItem } from "$lib/components/ui/select";
  import { Switch } from "$lib/components/ui/switch";
  import { Button } from "$lib/components/ui/button";
  import { ArrowLeft } from "@lucide/svelte";
  import type { Settings as SettingsShape, SortMetric } from "$lib/types";

  const intervalOptions = [1, 3, 6, 12, 24];

  let local = $state<SettingsShape>({ ...(store.settings as SettingsShape) });

  function save() {
    store.saveSettings(local);
    store.go("dashboard");
  }

  function back() {
    store.go("dashboard");
  }

  function onLanguageChange(l: string) {
    local.lang = l as Lang;
    setLanguage(l as Lang);
  }
</script>

<div class="flex flex-col gap-4 p-3">
  <div class="flex items-center gap-2">
    <Button variant="ghost" size="icon-sm" aria-label={$_("settings.back")} onclick={back}>
      <ArrowLeft size={16} />
    </Button>
    <h1 class="text-lg font-semibold">{$_("settings.title")}</h1>
  </div>

  <div class="flex flex-col gap-1.5">
    <span class="text-sm font-medium">{$_("settings.heroes_per_role")}: {local.top_n}</span>
    <Slider
      type="single"
      min={5}
      max={15}
      step={1}
      value={local.top_n}
      onValueChange={(v) => (local.top_n = v)}
    />
  </div>

  <div class="flex flex-col gap-1.5">
    <span class="text-sm font-medium">{$_("settings.sort")}</span>
    <Tabs
      value={local.sort}
      onValueChange={(v) => (local.sort = v as SortMetric)}
    >
      <TabsList>
        <TabsTrigger value="Pickrate">{$_("settings.pickrate")}</TabsTrigger>
        <TabsTrigger value="Winrate">{$_("settings.winrate")}</TabsTrigger>
      </TabsList>
    </Tabs>
  </div>

  <div class="flex flex-col gap-1.5">
    <span class="text-sm font-medium">{$_("settings.interval")}</span>
    <Select
      type="single"
      value={String(local.interval_hours)}
      onValueChange={(v) => (local.interval_hours = Number(v))}
    >
      <SelectTrigger>{local.interval_hours}h</SelectTrigger>
      <SelectContent>
        {#each intervalOptions as h (h)}
          <SelectItem value={String(h)}>{h}h</SelectItem>
        {/each}
      </SelectContent>
    </Select>
  </div>

  <div class="flex flex-col gap-1.5">
    <span class="text-sm font-medium">{$_("settings.account")}</span>
    <Select
      type="single"
      value={local.account_id ?? "all"}
      onValueChange={(v) => (local.account_id = v === "all" ? null : v)}
    >
      <SelectTrigger>{local.account_id ?? $_("settings.all_accounts")}</SelectTrigger>
      <SelectContent>
        <SelectItem value="all">{$_("settings.all_accounts")}</SelectItem>
        {#each store.accounts as a (a.id)}
          <SelectItem value={a.id}>{a.id}</SelectItem>
        {/each}
      </SelectContent>
    </Select>
  </div>

  <div class="flex items-center justify-between">
    <span class="text-sm font-medium">{$_("settings.autostart")}</span>
    <Switch checked={local.autostart} onCheckedChange={(v) => (local.autostart = v)} />
  </div>

  <div class="flex items-center justify-between">
    <span class="text-sm font-medium">{$_("settings.layout")}</span>
    <Switch checked={local.layout_columns} onCheckedChange={(v) => (local.layout_columns = v)} />
  </div>

  <div class="flex flex-col gap-1.5">
    <span class="text-sm font-medium">{$_("settings.language")}</span>
    <Select type="single" value={local.lang} onValueChange={onLanguageChange}>
      <SelectTrigger>{local.lang?.toUpperCase()}</SelectTrigger>
      <SelectContent>
        <SelectItem value="en">EN</SelectItem>
        <SelectItem value="ru">RU</SelectItem>
      </SelectContent>
    </Select>
  </div>

  <Button onclick={save}>{$_("settings.save")}</Button>
</div>
