<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import RoleColumn from "$lib/components/RoleColumn.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Button } from "$lib/components/ui/button";
  import { RefreshCw, LayoutGrid } from "@lucide/svelte";

  const skeletonSlots = [0, 1, 2, 3, 4];
  const busy = $derived(store.loading || store.status.kind === "Refreshing");
</script>

<div class="flex h-full flex-col">
  <StatusBar />

  <div class="scroll-thin min-h-0 flex-1 overflow-auto p-3">
    {#if store.loading && !store.snapshot}
      <div class="grid grid-cols-5 gap-3">
        {#each skeletonSlots as slot (slot)}
          <div class="flex flex-col gap-2 rounded-lg border border-border bg-card p-2">
            <Skeleton class="h-5 w-full" />
            {#each [0, 1, 2, 3, 4, 5] as row (row)}
              <Skeleton class="h-9 w-full" />
            {/each}
          </div>
        {/each}
      </div>
    {:else if store.snapshot}
      <div class="grid grid-cols-5 items-start gap-3">
        {#each store.snapshot.roles as role (role.position)}
          <RoleColumn {role} />
        {/each}
      </div>
    {:else}
      <div class="grid h-full place-items-center">
        <div class="flex max-w-xs flex-col items-center gap-4 text-center">
          <span class="grid size-12 place-items-center rounded-xl border border-border bg-card">
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
