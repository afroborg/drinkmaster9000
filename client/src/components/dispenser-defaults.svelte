<script lang="ts">
  import type { Dispenser } from '../models/dispenser';
  import Input from './input.svelte';

  export let dispenser: Dispenser;
</script>

<Input
  bind:value={dispenser.angle_between}
  type="number"
  label="Grader mellan dispensrar"
>
  <span slot="suffix">deg</span>
</Input>

<Input
  bind:value={dispenser.pour_speed_ml_ms}
  type="number"
  label="Tid för 1ml att rinna igenom"
>
  <span slot="suffix">ms/ml</span>
</Input>

<Input
  bind:value={dispenser.refill_delay_ms}
  type="number"
  label="Tid för att fylla på tom minidispenser"
>
  <span slot="suffix">ms</span>
</Input>

<Input
  bind:value={dispenser.rotation_delay_ms}
  type="number"
  label="Tid för rotation mellan 2 dispensrar"
>
  <span slot="suffix">ms</span>
</Input>

<Input bind:value={dispenser.waiting_angle} type="number" label="Väntevinkel">
  <span slot="suffix">deg</span>
</Input>

<style lang="scss">
</style>
