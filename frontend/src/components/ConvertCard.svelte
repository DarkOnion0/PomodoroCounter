<script lang="ts">
  import type { PomodoroCounter } from "../types";
  import type { TimeValue } from "./types";

  export let isPomodoro = true;
  export let active: boolean;
  export let value: number|TimeValue;
  export let counter: PomodoroCounter;

  $: if (!isPomodoro && value.minutes >= 60) {
    value.hours += Math.floor(value.minutes / 60);
    value.minutes = value.minutes % 60
  }
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
          <div class="stat-value">
            {#if Math.floor(counter.counter.spare_time / 60) > 0}{Math.floor(counter.counter.spare_time / 60)}hr{/if} {counter.counter.spare_time % 60}min
          </div>
        </div>

        <div class="stat">
          <div class="stat-title">Work Time</div>
          <div class="stat-value">
            {#if Math.floor(counter.counter.work_time / 60) > 0}{Math.floor(counter.counter.work_time / 60)}hr{/if} {counter.counter.work_time % 60}min
          </div>
        </div>

        <div class="stat">
          <div class="stat-title">Chill Time</div>
          <div class="stat-value">
            {#if Math.floor(counter.counter.chill_time / 60) > 0}{Math.floor(counter.counter.chill_time / 60)}hr{/if} {counter.counter.chill_time % 60}min
          </div>
        </div>
      </div>
    {/if}
  {:else}
    <h1 class="text-center">Time</h1>
    {#if active}
      <div class="grid grid-columns-2">
        <div class="form-control w-full max-w-xs">
          <label class="label">
            <span class="label-text">Hour(s)</span>
          </label>
          <input
            type="number"
            placeholder="Enter some hours"
            class="input input-bordered w-full max-w-xs"
            disabled={!active}
            bind:value={value.hours}
          />
        </div>
  
        <div class="form-control w-full max-w-xs">
          <label class="label">
            <span class="label-text">Minute(s)</span>
          </label>
          <input
            type="number"
            placeholder="Enter some minutes"
            class="input input-bordered w-full max-w-xs"
            disabled={!active}
            bind:value={value.minutes}
          />
        </div>
      </div>

    {:else}
      <div class="grid grid-cols-2">
        <div class="stat">
          <div class="stat-title">Work Time</div>
          <div class="stat-value">
            {#if Math.floor(counter.counter.work_time / 60) > 0}{Math.floor(counter.counter.work_time / 60)}hr{/if} {counter.counter.work_time % 60}min
          </div>
        </div>

        <div class="stat">
          <div class="stat-title">Chill Time</div>
          <div class="stat-value">
            {#if Math.floor(counter.counter.chill_time / 60) > 0}{Math.floor(counter.counter.chill_time / 60)}hr{/if} {counter.counter.chill_time % 60}min
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>
