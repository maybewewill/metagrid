<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { relTime } from "$lib/format";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, Settings, Play } from "@lucide/svelte";
  import * as ipc from "$lib/ipc";

  const dotClass = $derived.by(() => {
    switch (store.status.kind) {
      case "Ok":
        return "bg-emerald-500";
      case "Stale":
        return "bg-amber-500";
      case "Error":
        return "bg-red-500";
      case "Refreshing":
        return "bg-primary animate-pulse";
      default:
        return "bg-muted-foreground";
    }
  });

  const refreshing = $derived(store.status.kind === "Refreshing");
</script>

<div
  class="flex h-8 items-center justify-between gap-2 border-t border-border bg-card/60 px-3 text-xs text-muted-foreground"
>
  <div class="flex items-center gap-2">
    <span class={`size-2 rounded-full ${dotClass}`}></span>
    {#if store.snapshot}
      <span>{store.snapshot.patch}</span>
      <span>{relTime(store.snapshot.fetched_at)}</span>
    {/if}
  </div>
  <div class="flex items-center gap-1">
    <Button variant="ghost" size="sm" onclick={() => ipc.launchDota()}>
      <Play size={14} />
      {$_("app.play")}
    </Button>
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label={$_("status.refresh")}
      onclick={() => store.refresh()}
    >
      <RefreshCw size={14} class={refreshing ? "animate-spin" : ""} />
    </Button>
    <Button variant="ghost" size="icon-sm" aria-label="Settings" onclick={() => store.go("settings")}>
      <Settings size={14} />
    </Button>
  </div>
</div>
