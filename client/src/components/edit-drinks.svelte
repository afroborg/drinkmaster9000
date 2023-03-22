<script lang="ts">
  import { onMount } from 'svelte';
  import { getDrinks } from '../lib/api';
  import type { Drink } from '../models/drink';

  let drinks: Drink[] = [];

  onMount(async () => {
    drinks = await getDrinks();
  });
</script>

<div class="flex items-center gap-4">
  {#each drinks as drink}
    <div class="p-6 border border-gray-300 bg-gray-100 rounded-md w-full">
      <span class="font-medium">
        {drink.position}.
        {drink.name}
      </span>
    </div>
  {/each}
</div>

<style lang="scss">
</style>
