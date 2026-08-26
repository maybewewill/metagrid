<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import RoleColumn from "$lib/components/RoleColumn.svelte";
  import GridPreview from "$lib/components/GridPreview.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, LayoutGrid } from "@lucide/svelte";

  const skeletonSlots = [0, 1, 2, 3, 4];
  const busy = $derived(store.loading || store.status.kind === "Refreshing");
</script>

<div class="flex h-full flex-col">
  <StatusBar />

  <div class="scroll-thin min-h-0 flex-1 overflow-auto p-3.5">
    {#if (store.loading || store.status.kind === "Refreshing") && !store.snapshot}
      <div class="grid h-full grid-cols-5 gap-3">
        {#each skeletonSlots as slot (slot)}
          <div class="flex h-full min-w-0 flex-col overflow-hidden rounded-sm border border-zinc-800/80 bg-zinc-950/80 shadow-md">
            <!-- Header skeleton -->
            <div class="flex items-center gap-2 border-b border-zinc-800 bg-zinc-900/60 px-3.5 py-2.5">
              <Skeleton class="size-5 shrink-0 rounded-xs bg-zinc-800" />
              <Skeleton class="h-4 flex-1 rounded-xs bg-zinc-800" />
              <Skeleton class="h-3.5 w-10 shrink-0 rounded-xs bg-zinc-800" />
            </div>
            <!-- 7 Hero Card Skeletons matching exact HeroCard dimensions -->
            <div class="scroll-thin flex min-h-0 flex-1 flex-col justify-between gap-1 p-1.5 bg-zinc-950/40">
              {#each [0, 1, 2, 3, 4, 5, 6] as row (row)}
                <div class="flex flex-1 min-h-0 items-center overflow-hidden rounded-sm border border-zinc-800/60 bg-zinc-900/40">
                  <Skeleton class="h-full w-[52px] shrink-0 rounded-none bg-zinc-800/80 border-r border-zinc-800" />
                  <div class="flex min-w-0 flex-1 flex-col justify-center px-3.5 py-1 gap-1.5">
                    <Skeleton class="h-3.5 w-24 rounded-xs bg-zinc-800" />
                    <Skeleton class="h-2.5 w-16 rounded-xs bg-zinc-800/60" />
                  </div>
                  <div class="shrink-0 pr-3.5">
                    <Skeleton class="h-4 w-10 rounded-xs bg-zinc-800" />
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {:else if store.snapshot}
      {#if store.dashMode === "preview"}
        <GridPreview roles={store.snapshot.roles} />
      {:else}
        <div class={`grid h-full grid-cols-5 gap-3 transition-opacity duration-300 ${busy ? "opacity-60 pointer-events-none" : "opacity-100"}`}>
          {#each store.snapshot.roles as role, i (role.position)}
            <div class="mg-rise h-full min-h-0" style={`animation-delay:${i * 45}ms`}>
              <RoleColumn {role} />
            </div>
          {/each}
        </div>
      {/if}
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
