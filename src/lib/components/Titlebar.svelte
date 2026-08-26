<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Minus, X } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  let { patch }: { patch?: string } = $props();
  const win = getCurrentWindow();
</script>

<div
  class="flex h-9 items-center justify-between border-b border-border bg-card/60 select-none"
>
  <div data-tauri-drag-region class="flex h-full flex-1 items-center gap-2 px-3">
    <span class="text-sm font-semibold tracking-tight">MetaGrid</span>
    {#if patch}<Badge variant="secondary" class="text-[10px]">{patch}</Badge>{/if}
  </div>
  <div class="flex h-full">
    <button
      aria-label="Minimize"
      class="grid h-full w-11 place-items-center hover:bg-muted"
      onclick={() => win.minimize()}
    >
      <Minus size={15} />
    </button>
    <button
      aria-label="Close"
      class="grid h-full w-11 place-items-center hover:bg-primary hover:text-primary-foreground"
      onclick={() => win.hide()}
    >
      <X size={15} />
    </button>
  </div>
</div>
