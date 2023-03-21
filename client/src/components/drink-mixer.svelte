<script lang="ts">
  import Button from './button.svelte';
  import Glass from './glass.svelte';

  let isPouring: boolean = false;

  const togglePour = () => {
    isPouring = !isPouring;
  };
</script>

<div class="grid md:grid-cols-2 items-center gap-12">
  <Glass {isPouring} />

  <div>
    <Button on:click={togglePour}>
      {isPouring ? 'Stoppa' : 'Starta'}
    </Button>
  </div>
</div>

<style lang="scss">
</style>
