<script lang="ts">
  import config from '../config';

  let value: number = 0;

  const sendAngle = (a: number) => {
    fetch(`${config.api_url}/dispensers/0/angle/${a}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
  };

  const debounce = () => {
    let timeout: number;
    return (fn: () => void, delay: number) => {
      clearTimeout(timeout);
      timeout = setTimeout(fn, delay);
    };
  };

  const debounced = debounce();

  // $: debounced(() => sendAngle(value), 100);
</script>

<input type="range" bind:value min="0" max="180" />

<style lang="scss">
</style>
