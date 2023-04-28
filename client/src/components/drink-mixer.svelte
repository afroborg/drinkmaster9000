<script lang="ts">
  import { onMount } from 'svelte';
  import { getDispenser, getDrinks } from '../lib/api';
  import type { Drink } from '../models/drink';
  import Button from './button.svelte';
  import Glass from './glass.svelte';
  import Input from './input.svelte';
  import { useDispenser } from '../lib/use-timing';
  import type { Dispenser } from '../models/dispenser';

  let drinks: Drink[] = [];
  let dispenser: Dispenser = null;

  onMount(async () => {
    const [drinkResponse, dispenserResponse] = await Promise.all([
      getDrinks(),
      getDispenser(),
    ]);

    drinks = drinkResponse;
    dispenser = dispenserResponse;
  });

  $: ({ isPouring, dispenserAmount, createDrink, glassAmount, totalTime } =
    useDispenser(drinks, dispenser));
</script>

<div class="grid md:grid-cols-2 items-center gap-12">
  <Glass isPouring={$isPouring} timeToFillMs={$totalTime} />

  <div>
    <div
      class="bg-gray-300 p-4 rounded-md border border-gray-600 mb-4 flex items-center justify-between"
    >
      <span> Glasets storlek</span>

      <Input bind:value={$glassAmount} max="100" min="0" type="number">
        <span slot="suffix">ml</span>
      </Input>
    </div>

    <div class="flex flex-col gap-2 mb-4">
      {#each drinks as drink, i}
        <div
          class="px-4 py-2 bg-gray-200 rounded-md flex items-center gap-4 justify-between"
        >
          <span>{drink.name}</span>

          <Input
            bind:value={$dispenserAmount[i]}
            max="100"
            min="0"
            type="number"
          >
            <span slot="suffix">%</span>
          </Input>
        </div>
      {/each}
    </div>

    <div class="flex items-center gap-3">
      <Button on:click={$createDrink} isDisabled={$isPouring}>
        {$isPouring ? 'Stoppa' : 'Starta'}
      </Button>
    </div>
  </div>
</div>

<style lang="scss">
</style>
