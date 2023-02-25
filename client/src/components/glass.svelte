<script lang="ts">
  import { onMount } from 'svelte';

  let percentageFilled = 0;

  onMount(() => {
    const timeout = setInterval(() => {
      percentageFilled += 1;
      if (percentageFilled > 100) {
        percentageFilled = 0;
      }
    }, 100);

    return () => {
      clearTimeout(timeout);
    };
  });
</script>

<div class="flex gap-4 items-center">
  <div class="glass">
    <div class="water" style:--scale-y={percentageFilled} />
  </div>

  <div>
    <span class="font-bold">{percentageFilled} / 100%</span>
  </div>
</div>

<style lang="scss">
  .glass {
    width: 300px;
    height: 400px;
    background: white;
    clip-path: polygon(0 0, 100% 0, 85% 100%, 15% 100%);
    // linear gradient that looks like glass (in black)
    background: linear-gradient(
      to bottom,
      theme('colors.slate.600 / 5%'),
      theme('colors.slate.600 / 30%')
    );
  }

  .water {
    width: 100%;
    height: 100%;
    background: #67deff;
    transform: scaleY(calc(var(--scale-y, 1) / 100));
    transform-origin: bottom;
    transition: all 200ms linear;
  }
</style>
