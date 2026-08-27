<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { setLanguage, type Lang } from "$lib/i18n";
  import { Button } from "$lib/components/ui/button";
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { Separator } from "$lib/components/ui/separator";
  import { Select, SelectContent, SelectItem, SelectTrigger } from "$lib/components/ui/select";
  import { RefreshCw, Check, Info } from "@lucide/svelte";
  import * as ipc from "$lib/ipc";

  let selectedAccount = $state<string | null>(null);
  let lang = $state<Lang>((store.settings?.lang as Lang) ?? "en");
  let gridMode = $state<"separate" | "merge">("separate");
  let mergeTarget = $state<string | null>(null);
  let gridConfigs = $state<string[]>([]);
  let step = $state<"initial" | "mode_select">("initial");
  let fetching = $state(false);

  function pickLanguage(l: string) {
    lang = l as Lang;
    setLanguage(l as Lang);
  }

  async function fetchFirstMeta() {
    fetching = true;
    try {
      await store.fetchOnly();
      gridConfigs = await ipc.listGridConfigs().catch(() => []);
      if (gridConfigs.length > 0 && !mergeTarget) {
        mergeTarget = gridConfigs[0];
      }
      step = "mode_select";
    } finally {
      fetching = false;
    }
  }

  async function finish() {
    fetching = true;
    try {
      await store.saveSettings({
        onboarded: true,
        account_id: selectedAccount,
        lang,
        grid_mode: gridMode,
        merge_target: mergeTarget,
      });
      await store.refresh();
      store.go("dashboard");
    } finally {
      fetching = false;
    }
  }
</script>

<div class="scroll-thin h-full overflow-auto">
  <div class="mx-auto flex min-h-full max-w-sm flex-col justify-center gap-6 px-6 py-10">
    <div class="flex flex-col items-center gap-3 text-center">
      <span class="grid size-11 place-items-center rounded-sm bg-foreground text-background">
        <span class="text-lg font-bold leading-none">M</span>
      </span>
      <div class="flex flex-col gap-1">
        <h1 class="text-lg font-semibold text-balance">
          {step === "initial" ? $_("onboarding.welcome") : $_("onboarding.choose_mode")}
        </h1>
        <p class="text-sm text-balance text-muted-foreground">
          {step === "initial" ? $_("onboarding.subtitle") : $_("onboarding.choose_mode_desc")}
        </p>
      </div>
    </div>

    {#if step === "initial"}
      <div class="flex flex-col gap-5">
        <div class="flex items-center justify-between gap-4">
          <span class="text-sm font-medium">{$_("settings.language")}</span>
          <Tabs value={lang} onValueChange={pickLanguage}>
            <TabsList class="h-8">
              <TabsTrigger value="en" class="text-xs">EN</TabsTrigger>
              <TabsTrigger value="ru" class="text-xs">RU</TabsTrigger>
            </TabsList>
          </Tabs>
        </div>

        <Separator />

        <div class="flex flex-col gap-2">
          <span class="text-sm font-medium">{$_("onboarding.pick_account")}</span>
          {#if store.accounts.length === 0}
            <p class="text-sm text-balance text-muted-foreground">{$_("onboarding.no_accounts")}</p>
          {:else}
            <div class="flex flex-wrap gap-2">
              <Button
                variant={selectedAccount === null ? "default" : "outline"}
                size="sm"
                onclick={() => (selectedAccount = null)}
              >
                {$_("settings.all_accounts")}
              </Button>
              {#each store.accounts as a (a.id)}
                <Button
                  variant={selectedAccount === a.id ? "default" : "outline"}
                  size="sm"
                  class="font-mono"
                  onclick={() => (selectedAccount = a.id)}
                >
                  {a.id}
                </Button>
              {/each}
            </div>
          {/if}
        </div>
      </div>

      <div class="flex flex-col gap-2 pt-2">
        <Button class="w-full gap-2" disabled={fetching} onclick={fetchFirstMeta}>
          <RefreshCw size={15} class={fetching ? "animate-spin" : ""} />
          {$_("onboarding.fetch_meta")}
        </Button>
      </div>
    {:else}
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2.5">
          <button
            type="button"
            class="flex flex-col gap-1 rounded-sm border p-3 text-left transition-colors {gridMode === 'separate' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border bg-card/50 hover:bg-card'}"
            onclick={() => (gridMode = "separate")}
          >
            <div class="flex items-center justify-between">
              <span class="text-sm font-semibold text-foreground">{$_("settings.grid_mode_separate")}</span>
              {#if gridMode === "separate"}
                <Check size={16} class="text-primary" />
              {/if}
            </div>
            <span class="text-xs text-muted-foreground">{$_("settings.grid_mode_separate_hint")}</span>
          </button>

          <button
            type="button"
            class="flex flex-col gap-1 rounded-sm border p-3 text-left transition-colors {gridMode === 'merge' ? 'border-primary bg-primary/5 shadow-xs' : 'border-border bg-card/50 hover:bg-card'}"
            onclick={() => (gridMode = "merge")}
          >
            <div class="flex items-center justify-between">
              <span class="text-sm font-semibold text-foreground">{$_("settings.grid_mode_merge")}</span>
              {#if gridMode === "merge"}
                <Check size={16} class="text-primary" />
              {/if}
            </div>
            <span class="text-xs text-muted-foreground">{$_("settings.grid_mode_merge_hint")}</span>
          </button>
        </div>

        {#if gridMode === "merge"}
          <div class="flex flex-col gap-1.5 rounded-sm border border-border bg-card/40 p-3">
            <span class="text-xs font-semibold text-foreground">{$_("settings.merge_target")}</span>
            {#if gridConfigs.length === 0}
              <span class="text-xs text-muted-foreground">{$_("settings.merge_target_empty")}</span>
            {:else}
              <Select
                type="single"
                value={mergeTarget ?? ""}
                onValueChange={(v) => (mergeTarget = v || null)}
              >
                <SelectTrigger class="w-full truncate rounded-sm text-xs">
                  {mergeTarget ?? $_("settings.merge_target_placeholder")}
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

        <div class="flex items-start gap-2.5 rounded-sm border border-border/60 bg-muted/40 p-2.5 text-xs text-muted-foreground">
          <Info size={14} class="mt-0.5 shrink-0 text-muted-foreground" />
          <span>{$_("onboarding.tip_settings")}</span>
        </div>

        <Button class="w-full gap-2 mt-2" disabled={fetching} onclick={finish}>
          {#if fetching}
            <RefreshCw size={15} class="animate-spin" />
          {/if}
          {$_("onboarding.finish")}
        </Button>
      </div>
    {/if}
  </div>
</div>
