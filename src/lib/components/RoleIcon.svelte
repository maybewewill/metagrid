<script lang="ts">
  let {
    position,
    class: className = "size-4",
  }: {
    position: string | number;
    class?: string;
  } = $props();

  const posNum = $derived(
    typeof position === "number"
      ? position
      : Number(String(position).replace(/\D/g, "")) || 1
  );
</script>

{#if posNum === 1}
  <!-- POS 1: Safe Lane / Carry (Sword) -->
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    class="{className} shrink-0"
    fill="none"
  >
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M4.792 16.244L.623 20.388a2.107 2.107 0 000 2.992h.002a2.136 2.136 0 003.01 0l4.167-4.142-3.01-2.994z"
      fill="currentColor"
      fill-opacity="0.5"
    />
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M2.853 10.193c-.373.32-.597.78-.615 1.268-.018.49.17.964.517 1.309l8.53 8.478c.326.327.77.507 1.233.507a1.73 1.73 0 001.228-.51 1.717 1.717 0 00-.003-2.434c-.86-.855-1.857-1.843-1.857-1.843s8.881-7.06 10.836-8.612a1.18 1.18 0 00.43-.776c.17-1.423.668-5.646.845-7.124a.406.406 0 00-.119-.337.414.414 0 00-.34-.116l-6.767.843c-.304.038-.578.19-.77.427L7.134 12.245s-1.087-1.085-1.973-1.962a1.702 1.702 0 00-2.305-.09h-.003zm7.519 4.69l9.922-9.861a.79.79 0 10-1.124-1.116l-9.922 9.863a.782.782 0 000 1.114c.31.31.813.31 1.124 0z"
      fill="currentColor"
    />
  </svg>
{:else if posNum === 2}
  <!-- POS 2: Mid Lane (Crossed Swords) -->
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    class="{className} shrink-0"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5" fill="currentColor" fill-opacity="0.15" />
    <line x1="13" y1="19" x2="19" y2="13" />
    <line x1="16" y1="16" x2="20" y2="20" />
    <line x1="19" y1="21" x2="21" y2="19" />
    <polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5" fill="currentColor" fill-opacity="0.15" />
    <line x1="5" y1="14" x2="9" y2="18" />
    <line x1="7" y1="4" x2="4" y2="7" />
    <line x1="3" y1="5" x2="5" y2="3" />
  </svg>
{:else if posNum === 3}
  <!-- POS 3: Offlane (Shield) -->
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    class="{className} shrink-0"
    fill="currentColor"
    fill-opacity="0.2"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
    <path d="M12 2v20" stroke-opacity="0.5" stroke-dasharray="2 2" />
  </svg>
{:else if posNum === 4}
  <!-- POS 4: Soft Support (Flask / Utility) -->
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    class="{className} shrink-0"
    fill="currentColor"
    fill-opacity="0.2"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path d="M10 2v3a6 6 0 0 0-1.8 4.2L4 18.5A2.5 2.5 0 0 0 6.2 22h11.6a2.5 2.5 0 0 0 2.2-3.5l-4.2-9.3A6 6 0 0 0 14 5V2" />
    <line x1="8.5" y1="2" x2="15.5" y2="2" />
    <path d="M7 16h10" stroke-opacity="0.6" />
  </svg>
{:else}
  <!-- POS 5: Hard Support (Observer Ward / Eye) -->
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    class="{className} shrink-0"
    fill="currentColor"
    fill-opacity="0.2"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path d="M2 12s3.6-7 10-7 10 7 10 7-3.6 7-10 7-10-7-10-7Z" />
    <circle cx="12" cy="12" r="3" fill="currentColor" />
  </svg>
{/if}
