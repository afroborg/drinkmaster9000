<script lang="ts">
  import { onMount } from 'svelte';
  import { getDispenser, updateDispenser } from '../lib/api';
  import type { Dispenser } from '../models/dispenser';
  import ServoSettings from './servo-settings.svelte';
  import DispenserDefaults from './dispenser-defaults.svelte';
  import Button from './button.svelte';
  import toast from 'svelte-french-toast';

  let dispenser: Dispenser = null;

  onMount(async () => {
    dispenser = await getDispenser();
  });

  const handleSave = async () => {
    try {
      await updateDispenser(dispenser);
      toast.success('Dispensern sparades');
    } catch (error) {
      toast.error('Något gick fel');
    }
  };
</script>

{#if dispenser}
  <form class="space-y-8" on:submit|preventDefault={handleSave}>
    <div class="grid md:grid-cols-5 gap-2">
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

    <Button isSubmit>Spara dispenser</Button>
  </form>
{/if}

<style lang="scss">
</style>
