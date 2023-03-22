<script lang="ts">
  import { onMount } from 'svelte';
  import { getDrinks } from '../lib/api';
  import type { Drink } from '../models/drink';
  import Button from './button.svelte';

  let drinks: Drink[] = [];

  onMount(async () => {
    drinks = await getDrinks();
  });

  const handleSave = async () => {
    console.log('save');
  };
</script>

<div class="space-y-4">
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

  <Button>Spara drycker</Button>
</div>

<style lang="scss">
</style>
