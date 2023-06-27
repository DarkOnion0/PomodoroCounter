<script lang="ts">
  export let isPomodoro = true;
  export let active: boolean;
  export let value: number;
</script>

<div class="w-full lg:w-1/3 prose flex items-center justify-center flex-col" {...$$props.class}>
  {#if isPomodoro}
    <h1 class="text-center">Pomodoro</h1>
    <input
      type="number"
      placeholder="Enter a number of pomodoro"
      class="input input-bordered w-full max-w-xs"
      min="1"
      disabled={!active}
      bind:value
    />
  {:else}
    <h1 class="text-center">Time</h1>
    <input
      type="number"
      placeholder="Enter some minutes"
      class="input input-bordered w-full max-w-xs"
      disabled={!active}
      bind:value
    />
  {/if}
</div>
