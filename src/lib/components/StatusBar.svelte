<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { relTime } from "$lib/format";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, Settings2 } from "@lucide/svelte";

  const kind = $derived(store.status.kind);
  const refreshing = $derived(kind === "Refreshing");

  const dot = $derived.by(() => {
    switch (kind) {
      case "Ok":
        return "bg-success";
      case "Stale":
        return "bg-warning";
      case "Error":
        return "bg-destructive";
      case "Refreshing":
        return "bg-foreground animate-pulse";
      default:
        return "bg-muted-foreground/50";
    }
  });

  const label = $derived.by(() => {
    switch (kind) {
      case "Ok":
        return $_("status.fresh");
      case "Stale":
        return $_("status.stale");
      case "Error":
        return $_("status.error");
      case "Refreshing":
        return $_("status.refreshing");
      default:
        return "";
    }
  });
</script>

<div
  class="flex h-10 shrink-0 items-center justify-between gap-3 border-b border-border bg-background px-3"
>
  <div class="flex min-w-0 items-center gap-2 text-xs">
    <span class={`size-2 shrink-0 rounded-full ${dot}`}></span>
    <span class="font-medium text-foreground">{label}</span>
    {#if store.snapshot}
      <span class="text-muted-foreground/50">·</span>
      <span class="truncate text-muted-foreground">
        {$_("status.updated", { values: { time: relTime(store.snapshot.fetched_at) } })}
      </span>
    {/if}
  </div>

  <div class="flex items-center gap-1">
    <Button
      variant="outline"
      size="sm"
      class="h-7 gap-1.5 px-2.5 text-xs"
      disabled={refreshing}
      onclick={() => store.refresh()}
    >
      <RefreshCw size={13} class={refreshing ? "animate-spin" : ""} />
      {$_("status.refresh")}
    </Button>
    <Button
      variant="ghost"
      size="icon-sm"
      class="size-7"
      aria-label={$_("settings.title")}
      onclick={() => store.go("settings")}
    >
      <Settings2 size={15} />
    </Button>
  </div>
</div>
