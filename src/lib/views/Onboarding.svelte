<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { setLanguage, type Lang } from "$lib/i18n";
  import { Button } from "$lib/components/ui/button";
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { Separator } from "$lib/components/ui/separator";
  import { RefreshCw, Check } from "@lucide/svelte";

  let selectedAccount = $state<string | null>(null);
  let lang = $state<Lang>((store.settings?.lang as Lang) ?? "en");
  let fetching = $state(false);
  let fetched = $state(false);

  function pickLanguage(l: string) {
    lang = l as Lang;
    setLanguage(l as Lang);
  }

  async function fetchFirstMeta() {
    fetching = true;
    try {
      await store.refresh();
      fetched = true;
    } finally {
      fetching = false;
    }
  }

  async function finish() {
    await store.saveSettings({ onboarded: true, account_id: selectedAccount, lang });
    store.go("dashboard");
  }
</script>

<div class="scroll-thin h-full overflow-auto">
  <div class="mx-auto flex min-h-full max-w-sm flex-col justify-center gap-7 px-6 py-10">
    <div class="flex flex-col items-center gap-3 text-center">
      <span class="grid size-11 place-items-center rounded-sm bg-foreground text-background">
        <span class="text-lg font-bold leading-none">M</span>
      </span>
      <div class="flex flex-col gap-1">
        <h1 class="text-lg font-semibold text-balance">{$_("onboarding.welcome")}</h1>
        <p class="text-sm text-balance text-muted-foreground">{$_("onboarding.subtitle")}</p>
      </div>
    </div>

    <div class="flex flex-col gap-5">
      <!-- Language -->
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

      <!-- Account -->
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

    <div class="flex flex-col gap-2">
      <Button variant="outline" class="gap-2" disabled={fetching} onclick={fetchFirstMeta}>
        {#if fetched && !fetching}
          <Check size={15} />
          {$_("onboarding.fetched")}
        {:else}
          <RefreshCw size={15} class={fetching ? "animate-spin" : ""} />
          {$_("onboarding.fetch")}
        {/if}
      </Button>
      <Button class="w-full" onclick={finish}>{$_("onboarding.finish")}</Button>
    </div>
  </div>
</div>
