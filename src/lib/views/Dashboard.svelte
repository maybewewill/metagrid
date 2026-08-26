<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import RoleColumn from "$lib/components/RoleColumn.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Button } from "$lib/components/ui/button";

  const skeletonSlots = [0, 1, 2, 3, 4];
</script>

<div class="flex h-full flex-col">
  <StatusBar />
  <div class="flex-1 overflow-auto p-3">
    {#if store.loading && !store.snapshot}
      <div class="grid grid-cols-5 gap-3">
        {#each skeletonSlots as slot (slot)}
          <Skeleton class="h-72 rounded-xl" />
        {/each}
      </div>
    {:else if store.snapshot}
      <div class="grid grid-cols-5 gap-3">
        {#each store.snapshot.roles as role (role.position)}
          <RoleColumn {role} />
        {/each}
      </div>
    {:else}
      <div class="grid h-full place-items-center">
        <Button onclick={() => store.refresh()}>{$_("empty.cta")}</Button>
      </div>
    {/if}
  </div>
</div>
