<script lang="ts">
  import type { Servo } from '../models/servo';
  import Checkbox from './checkbox.svelte';
  import Input from './input.svelte';

  export let servo: Servo;
</script>

<div>
  <div class="grid md:grid-cols-7 gap-4">
    <Input bind:value={servo.pin} type="number" label="Pin" />

    <Input bind:value={servo.min_us} type="number" label="Minimum µs" />

    <Input bind:value={servo.max_us} type="number" label="Maximum µs" />

    <Input bind:value={servo.neutral_us} type="number" label="Neutral µs" />

    <Input bind:value={servo.start_angle} type="number" label="Startvinkel" />

    <Input bind:value={servo.end_angle} type="number" label="Slutvinkel" />

    <div class="mt-9">
      <Checkbox bind:value={servo.is_reversed} label="Går baklänges" />
    </div>
  </div>
</div>

<style lang="scss">
</style>
