<script lang="ts">
  import type { PomodoroCounter } from "../types";

  export let isPomodoro = true;
  export let active: boolean;
  export let value: number;
  export let counter: PomodoroCounter;
</script>

<div
  class="w-full lg:w-1/3 prose flex items-center justify-center flex-col"
  {...$$props.class}
>
  {#if isPomodoro}
    <h1 class="text-center">Pomodoro</h1>
    {#if active}
      <input
        type="number"
        placeholder="Enter a number of pomodoro"
        class="input input-bordered w-full max-w-xs"
        min="1"
        disabled={!active}
        bind:value
      />
    {:else}
      <div class="grid grid-cols-2">
        <div class="stat">
          <div class="stat-title">Pomodoro</div>
          <div class="stat-value">{counter.pomodoro}</div>
        </div>

        <div class="stat">
          <div class="stat-title">Spare Time</div>
          <div class="stat-value">{counter.counter.spare_time} min</div>
        </div>

        <div class="stat">
          <div class="stat-title">Work Time</div>
          <div class="stat-value">{counter.counter.work_time} min</div>
        </div>

        <div class="stat">
          <div class="stat-title">Chill Time</div>
          <div class="stat-value">{counter.counter.chill_time} min</div>
        </div>
      </div>
    {/if}
  {:else}
    <h1 class="text-center">Time</h1>
    {#if active}
      <input
        type="number"
        placeholder="Enter some minutes"
        class="input input-bordered w-full max-w-xs"
        disabled={!active}
        bind:value
      />
    {:else}
      <div class="grid grid-cols-2">
        <div class="stat">
          <div class="stat-title">Work Time</div>
          <div class="stat-value">{counter.counter.work_time} min</div>
        </div>

        <div class="stat">
          <div class="stat-title">Chill Time</div>
          <div class="stat-value">{counter.counter.chill_time} min</div>
        </div>
      </div>
    {/if}
  {/if}
</div>
