<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { Minus, X } from "@lucide/svelte";

  let { patch }: { patch?: string } = $props();
  const win = getCurrentWindow();
</script>

<header
  class="flex h-9 shrink-0 items-center justify-between border-b border-border bg-background select-none"
>
  <div data-tauri-drag-region class="flex h-full flex-1 items-center gap-2.5 pl-3">
    <span class="grid size-4 place-items-center rounded-sm bg-foreground text-background">
      <span class="text-[10px] font-bold leading-none">M</span>
    </span>
    <span class="text-[13px] font-semibold tracking-tight">MetaGrid</span>
    {#if patch}
      <span
        class="rounded-sm border border-border px-1.5 py-px font-mono text-[10px] leading-tight text-muted-foreground"
      >
        {patch}
      </span>
    {/if}
  </div>
  <div class="flex h-full no-drag">
    <button
      type="button"
      aria-label="Minimize"
      class="grid h-full w-11 place-items-center text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
      onclick={() => win.minimize()}
    >
      <Minus size={15} />
    </button>
    <button
      type="button"
      aria-label="Close"
      class="grid h-full w-11 place-items-center text-muted-foreground transition-colors hover:bg-destructive hover:text-destructive-foreground"
      onclick={() => win.hide()}
    >
      <X size={15} />
    </button>
  </div>
</header>
