<script lang="ts">
  import { onMount } from 'svelte';
  import { getDrinks } from '../lib/api';
  import type { Drink } from '../models/api';

  let drinks: Drink[] = [];

  onMount(async () => {
    drinks = await getDrinks();
  });
</script>

<div class="edit-drinks">
  {#each drinks as drink}
    <div>{drink.name}</div>
  {/each}
</div>

<style lang="scss">
</style>
