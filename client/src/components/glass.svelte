<script lang="ts">
  import { onDestroy } from 'svelte';

  export let isPouring = false;

  let percentageFilled = 0;
  let audio: HTMLAudioElement;
  let timeout: number;

  const mockPour = () => {
    timeout = setInterval(() => {
      percentageFilled += 1;
      if (percentageFilled > 100) {
        percentageFilled = 0;
      }
    }, 100);
  };

  $: startPour(isPouring);

  const startPour = (doPour: boolean) => {
    if (doPour) {
      audio.play();
      mockPour();
    } else {
      percentageFilled = 0;
      if (audio) {
        audio.pause();
        audio.currentTime = 0;
      }
      clearTimeout(timeout);
    }
  };

  onDestroy(() => {
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

<audio bind:this={audio} hidden playsinline loop>
  <source src="/water-sound.mp3" type="audio/mp3" />
  Your browser does not support the audio element.
</audio>

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
