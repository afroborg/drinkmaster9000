<script lang="ts">
  export let value: number | string;
</script>

<div
  class="flex items-center bg-gray-100 rounded-md pl-4 focus-within:border-gray-600 border border-gray-300 overflow-hidden"
  class:pr-4={!$$slots.suffix}
>
  <input
    class=" h-full w-32 bg-transparent outline-none"
    class:py-3={!$$slots.suffix}
    bind:value
  />

  {#if $$slots.suffix}
    <div
      class="px-4 bg-gray-600 h-full py-3 text-white text-sm outline-none border-none"
    >
      <slot name="suffix" />
    </div>
  {/if}
</div>

<style lang="scss">
</style>
