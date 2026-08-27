<script lang="ts">
  import { _, locale } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import { relTime } from "$lib/format";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, Settings2 } from "@lucide/svelte";

  const kind = $derived(store.status.kind);
  const refreshing = $derived(kind === "Refreshing" || store.loading);

  const dot = $derived.by(() => {
    if (refreshing) {
      return "bg-primary animate-pulse";
    }
    switch (kind) {
      case "Ok":
        return "bg-success";
      case "Stale":
        return "bg-warning";
      case "Error":
        return "bg-destructive";
      default:
        return "bg-muted-foreground/50";
    }
  });

  const label = $derived.by(() => {
    if (refreshing) {
      return store.fetchingOnly ? $_("status.fetching") : $_("status.refreshing");
    }
    switch (kind) {
      case "Ok":
        return $_("status.fresh");
      case "Stale":
        return $_("status.stale");
      case "Error":
        return $_("status.error");
      default:
        return "";
    }
  });

  const errorDetail = $derived(store.status.kind === "Error" ? store.status.detail : undefined);
</script>

<div
  class="flex h-11 shrink-0 items-center justify-between gap-3 border-b border-border bg-background px-3.5"
>
  <div class="flex min-w-0 items-center gap-2 text-[13px]">
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
          {$_("status.updated", { values: { time: relTime(store.snapshot.fetched_at, new Date(), $locale ?? "en") } })}
        </span>
      {/if}
    {/if}
  </div>

  <div class="flex items-center gap-2">
    <Button
      variant="outline"
      size="sm"
      class="h-8 gap-2 rounded-sm px-3 text-xs font-semibold"
      disabled={refreshing}
      onclick={() => store.refresh()}
    >
      <RefreshCw size={14} class={`transition-transform duration-500 ${refreshing ? "animate-spin text-primary" : ""}`} />
      {$_("status.refresh")}
    </Button>
    <Button
      variant="ghost"
      size="icon-sm"
      class="size-8 rounded-sm"
      aria-label={$_("settings.title")}
      title={$_("settings.title")}
      onclick={() => store.go("settings")}
    >
      <Settings2 size={16} />
    </Button>
  </div>
</div>
