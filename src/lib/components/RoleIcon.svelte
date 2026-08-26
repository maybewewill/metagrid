<script lang="ts">
  import pos1 from "$lib/assets/pos/pos_1.png";
  import pos2 from "$lib/assets/pos/pos_2.png";
  import pos3 from "$lib/assets/pos/pos_3.png";
  import pos4 from "$lib/assets/pos/pos_4.png";
  import pos5 from "$lib/assets/pos/pos_5.png";

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

  const iconSrc = $derived.by(() => {
    switch (posNum) {
      case 1:
        return pos1;
      case 2:
        return pos2;
      case 3:
        return pos3;
      case 4:
        return pos4;
      case 5:
        return pos5;
      default:
        return pos1;
    }
  });
</script>

<img
  src={iconSrc}
  alt="POS {posNum}"
  class="{className} shrink-0 object-contain select-none"
  loading="lazy"
  draggable="false"
/>
