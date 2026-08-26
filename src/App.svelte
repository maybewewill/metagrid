<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { _ } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { store } from "$lib/store.svelte";
  import Titlebar from "$lib/components/Titlebar.svelte";
  import Dashboard from "$lib/views/Dashboard.svelte";
  import Settings from "$lib/views/Settings.svelte";
  import Onboarding from "$lib/views/Onboarding.svelte";
  import { Toaster } from "$lib/components/ui/sonner";
  import { onRefreshDone, onRefreshError } from "$lib/ipc";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  onMount(() => {
    store.init();

    let unlistenDone: UnlistenFn | undefined;
    let unlistenError: UnlistenFn | undefined;

    onRefreshDone(() => toast.success($_("toast.updated"))).then((fn) => {
      unlistenDone = fn;
    });
    onRefreshError(() => toast.error($_("toast.error"))).then((fn) => {
      unlistenError = fn;
    });

    return () => {
      unlistenDone?.();
      unlistenError?.();
    };
  });
</script>

<div class="flex h-full flex-col">
  <Titlebar patch={store.snapshot?.patch} />
  <main class="flex-1 overflow-auto p-3">
    {#key store.view}
      <div in:fly={{ y: 8, duration: 150 }}>
        {#if store.view === "onboarding"}
          <Onboarding />
        {:else if store.view === "settings"}
          <Settings />
        {:else}
          <Dashboard />
        {/if}
      </div>
    {/key}
  </main>
  <Toaster position="bottom-right" />
</div>
