<script lang="ts">
  export let isSubmit: boolean = false;
</script>

<button
  on:click
  class="p-4 py-2 bg-black text-white rounded-md"
  type={isSubmit ? 'submit' : 'button'}
>
  <slot />
</button>

<style lang="scss">
</style>
