<script lang="ts">
  export let label: string = null;
  export let value: boolean;
</script>

<div class="flex gap-2 items-center">
  <input
    id="default-checkbox"
    type="checkbox"
    value=""
    class="w-6 h-6"
    bind:checked={value}
  />
  {#if label}
    <span class="font-semibold text-sm text-gray-800">
      {label}
    </span>
  {/if}
</div>

<style lang="scss">
</style>
