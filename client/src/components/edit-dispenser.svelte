<script lang="ts">
  import { onMount } from 'svelte';
  import { getDispenser } from '../lib/api';
  import type { Dispenser } from '../models/dispenser';
  import ServoSettings from './servo-settings.svelte';
  import DispenserDefaults from './dispenser-defaults.svelte';

  let dispenser: Dispenser = null;

  onMount(async () => {
    dispenser = await getDispenser();
  });
</script>

{#if dispenser}
  <div class="space-y-8">
    <div class="grid md:grid-cols-3 gap-4">
      <DispenserDefaults bind:dispenser />
    </div>

    <div class="space-y-2">
      <h2 class="font-semibold text-lg">Rotationsmojängen</h2>
      <ServoSettings bind:servo={dispenser.cup_rotator} />
    </div>

    <div class="space-y-2">
      <h2 class="font-semibold text-lg">Tryckmojängen</h2>
      <div class="space-y-4">
        {#each dispenser.pusher as pusher, i}
          <div class="space-y-1">
            <span class="font-semibold">Tryckare {i + 1}</span>
            <ServoSettings bind:servo={pusher} />
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
</style>
