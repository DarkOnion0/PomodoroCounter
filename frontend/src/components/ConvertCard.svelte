<script lang="ts">
  export let isPomodoro = true;
  export let active: boolean;
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
    />
  {:else}
    <h1 class="text-center">Time</h1>
    <input
      type="time"
      class="input input-bordered w-full max-w-xs"
      disabled={!active}
    />
  {/if}
</div>
