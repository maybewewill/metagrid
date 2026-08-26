<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { setLanguage, type Lang } from "$lib/i18n";
  import { Button } from "$lib/components/ui/button";

  let selectedAccount = $state<string | null>(null);
  let fetching = $state(false);

  function pickLanguage(l: Lang) {
    setLanguage(l);
  }

  async function fetchFirstMeta() {
    fetching = true;
    try {
      await store.refresh();
    } finally {
      fetching = false;
    }
  }

  async function finish() {
    await store.saveSettings({ onboarded: true, account_id: selectedAccount });
    store.go("dashboard");
  }
</script>

<div class="mx-auto flex h-full max-w-md flex-col justify-center gap-6 p-4">
  <h1 class="text-xl font-semibold">{$_("onboarding.welcome")}</h1>

  <section class="flex flex-col gap-2">
    <span class="text-sm font-medium text-muted-foreground">{$_("settings.language")}</span>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => pickLanguage("en")}>EN</Button>
      <Button variant="outline" onclick={() => pickLanguage("ru")}>RU</Button>
    </div>
  </section>

  <section class="flex flex-col gap-2">
    <span class="text-sm font-medium text-muted-foreground">{$_("onboarding.pick_account")}</span>
    {#if store.accounts.length === 0}
      <p class="text-sm text-muted-foreground">{$_("onboarding.no_accounts")}</p>
    {:else}
      <div class="flex flex-wrap gap-2">
        <Button
          variant={selectedAccount === null ? "default" : "outline"}
          onclick={() => (selectedAccount = null)}
        >
          {$_("settings.all_accounts")}
        </Button>
        {#each store.accounts as a (a.id)}
          <Button
            variant={selectedAccount === a.id ? "default" : "outline"}
            onclick={() => (selectedAccount = a.id)}
          >
            {a.id}
          </Button>
        {/each}
      </div>
    {/if}
  </section>

  <section class="flex flex-col gap-2">
    <Button variant="secondary" onclick={fetchFirstMeta} disabled={fetching}>
      {$_("onboarding.fetch")}
    </Button>
  </section>

  <Button onclick={finish}>{$_("onboarding.finish")}</Button>
</div>
