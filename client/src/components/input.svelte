<script lang="ts">
  export let label: string = null;
  export let value: number | string;
  export let max: number | string = null;
  export let min: number | string = null;
  export let type: 'text' | 'number' = 'text';

  const handleInput: svelte.JSX.ChangeEventHandler<HTMLInputElement> = (e) => {
    if (type === 'number') {
      value = +e.currentTarget.value;
      return;
    }

    value = e.currentTarget.value;
  };
</script>

<div class="space-y-1">
  {#if label}
    <span class="font-semibold text-sm text-gray-800">
      {label}
    </span>
  {/if}

  <div
    class="flex items-center bg-gray-100 rounded-md pl-4 focus-within:border-gray-600 border border-gray-300 overflow-hidden"
    class:pr-4={!$$slots.suffix}
  >
    <input
      class="h-full w-full bg-transparent outline-none"
      class:py-3={!$$slots.suffix}
      {type}
      {max}
      {min}
      {value}
      on:input={handleInput}
    />

    {#if $$slots.suffix}
      <div
        class="px-4 bg-gray-600 h-full py-3 text-white text-sm outline-none border-none"
      >
        <slot name="suffix" />
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
</style>
