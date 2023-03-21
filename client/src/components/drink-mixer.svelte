<script lang="ts">
  import { onMount } from 'svelte';
  import config from '../config';
  import type { Dispenser } from '../models/dispenser';
  import Button from './button.svelte';
  import Glass from './glass.svelte';
  import Input from './input.svelte';

  let isPouring: boolean = false;

  let dispensers: Dispenser[] = [];
  let glassAmount: number = 33;
  let dispenserAmount: number[] = [];

  onMount(async () => {
    const res = await fetch(`${config.api_url}/dispensers`);
    dispensers = await res.json();
    dispenserAmount = dispensers.map(() => 0);
  });

  const togglePour = () => {
    isPouring = !isPouring;
  };
</script>

<div class="grid md:grid-cols-2 items-center gap-12">
  <Glass {isPouring} />

  <div>
    <div
      class="bg-gray-300 p-4 rounded-md border border-gray-600 mb-4 flex items-center justify-between"
    >
      <span> Glasets storlek</span>

      <Input bind:value={glassAmount}>
        <span slot="suffix">cl</span>
      </Input>
    </div>

    <div class="flex flex-col gap-2 mb-4">
      {#each dispensers as dispenser, i}
        <div
          class="px-4 py-2 bg-gray-200 rounded-md flex items-center gap-4 justify-between"
        >
          <span>{dispenser.name}</span>

          <Input bind:value={dispenserAmount[i]}>
            <span slot="suffix">%</span>
          </Input>
        </div>
      {/each}
    </div>

    <Button on:click={togglePour}>
      {isPouring ? 'Stoppa' : 'Starta'}
    </Button>
  </div>
</div>

<style lang="scss">
</style>
