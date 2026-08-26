<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { relTime } from "$lib/format";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, Settings2, List, LayoutGrid } from "@lucide/svelte";

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

  const errorDetail = $derived(store.status.kind === "Error" ? store.status.detail : undefined);
</script>

<div
  class="flex h-10 shrink-0 items-center justify-between gap-3 border-b border-border bg-background px-3"
>
  <div class="flex min-w-0 items-center gap-2 text-xs">
    <span class={`size-2 shrink-0 rounded-full ${dot}`}></span>
    {#if errorDetail}
      <span class="font-medium text-destructive truncate max-w-xs" title={errorDetail}>
        {errorDetail}
      </span>
    {:else}
      <span class="font-medium text-foreground">{label}</span>
      {#if store.snapshot}
        <span class="text-muted-foreground/50">·</span>
        <span class="truncate text-muted-foreground">
          {$_("status.updated", { values: { time: relTime(store.snapshot.fetched_at) } })}
        </span>
      {/if}
    {/if}
  </div>

  <div class="flex items-center gap-1.5">
    <div class="flex items-center rounded-sm border border-border p-0.5">
      <button
        type="button"
        aria-label={$_("status.list")}
        title={$_("status.list")}
        class={`grid size-6 place-items-center rounded-sm transition-colors ${store.dashMode === "list" ? "bg-secondary text-foreground" : "text-muted-foreground hover:text-foreground"}`}
        onclick={() => (store.dashMode = "list")}
      >
        <List size={14} />
      </button>
      <button
        type="button"
        aria-label={$_("status.grid")}
        title={$_("status.grid")}
        class={`grid size-6 place-items-center rounded-sm transition-colors ${store.dashMode === "preview" ? "bg-secondary text-foreground" : "text-muted-foreground hover:text-foreground"}`}
        onclick={() => (store.dashMode = "preview")}
      >
        <LayoutGrid size={14} />
      </button>
    </div>
    <Button
      variant="outline"
      size="sm"
      class="h-7 gap-1.5 rounded-sm px-2.5 text-xs"
      disabled={refreshing}
      onclick={() => store.refresh()}
    >
      <RefreshCw size={13} class={refreshing ? "animate-spin" : ""} />
      {$_("status.refresh")}
    </Button>
    <Button
      variant="ghost"
      size="icon-sm"
      class="size-7 rounded-sm"
      aria-label={$_("settings.title")}
      title={$_("settings.title")}
      onclick={() => store.go("settings")}
    >
      <Settings2 size={15} />
    </Button>
  </div>
</div>
