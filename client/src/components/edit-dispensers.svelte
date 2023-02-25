<script lang="ts">
  import { onMount } from 'svelte';
  import config from '../config';
  import type { Dispenser } from '../models/dispenser';

  let dispensers: Dispenser[] = [];

  onMount(async () => {
    const res = await fetch(`${config.api_url}/dispensers`);
    console.log(res);
    dispensers = await res.json();
  });
</script>

<div class="edit-dispensers">
  <div>
    <h1 class="font-semibold text-2xl">Dispensers</h1>

    <div class="flex gap-2 mt-2">
      {#each dispensers as dispenser}
        <div
          class="p-8 bg-slate-100 flex-1 rounded-md hover:shadow-md transition-all cursor-pointer"
        >
          <h2>
            <span class="font-bold">{dispenser.position}. </span>
            {dispenser.name}
          </h2>
        </div>
      {/each}
    </div>
  </div>
</div>

<style lang="scss">
</style>
