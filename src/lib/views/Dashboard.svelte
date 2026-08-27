<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import GridPreview from "$lib/components/GridPreview.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, LayoutGrid } from "@lucide/svelte";

  const busy = $derived(store.loading || store.status.kind === "Refreshing");
</script>

<div class="flex h-full flex-col">
  <StatusBar />

  <div class="scroll-thin min-h-0 flex-1 overflow-auto p-3.5">
    {#if (store.loading || store.status.kind === "Refreshing") && !store.snapshot}
      <div class="flex flex-col gap-4">
        <div class="flex items-center gap-2 border-b border-zinc-800/80 pb-2.5">
          <Skeleton class="h-7 w-20 rounded-xs bg-zinc-800" />
          <Skeleton class="h-7 w-16 rounded-xs bg-zinc-800/60" />
          <Skeleton class="h-7 w-16 rounded-xs bg-zinc-800/60" />
          <Skeleton class="h-7 w-16 rounded-xs bg-zinc-800/60" />
        </div>
        {#each [0, 1, 2] as sec (sec)}
          <div class="flex flex-col gap-2 rounded-sm border border-zinc-800/60 bg-zinc-950/60 p-3">
            <Skeleton class="h-4 w-28 rounded-xs bg-zinc-800" />
            <div class="flex flex-wrap gap-2 pt-1">
              {#each [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] as item (item)}
                <Skeleton class="h-[78px] w-[54px] rounded-sm bg-zinc-800/70" />
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {:else if store.snapshot}
      <div class={`transition-opacity duration-300 ${busy ? "opacity-60 pointer-events-none" : "opacity-100"}`}>
        <GridPreview snapshot={store.snapshot} />
      </div>
    {:else}
      <div class="grid h-full place-items-center">
        <div class="flex max-w-xs flex-col items-center gap-4 text-center">
          <span class="grid size-12 place-items-center rounded-sm border border-border bg-card">
            <LayoutGrid size={22} class="text-muted-foreground" />
          </span>
          <div class="flex flex-col gap-1">
            <h2 class="text-base font-semibold">{$_("empty.title")}</h2>
            <p class="text-sm text-balance text-muted-foreground">{$_("empty.desc")}</p>
          </div>
          <Button class="gap-2" disabled={busy} onclick={() => store.refresh()}>
            <RefreshCw size={15} class={busy ? "animate-spin" : ""} />
            {$_("empty.cta")}
          </Button>
        </div>
      </div>
    {/if}
  </div>
</div>
