<script lang="ts">
  import ConvertCard from "../components/ConvertCard.svelte";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { ArrowsRightLeft } from "@steeze-ui/heroicons";
  import { Pomodoro, Counter } from "./../wasm/pomolib";
  import type { PomodoroCounter } from "../types";

  let isPomodoroActive = true;

  let pomodoroCount: number;
  let timeCount: number;
  let counter: PomodoroCounter = {
    pomodoro: 0,
    counter: new Counter(),
  };

  $: if (isPomodoroActive) {
    let tmp = Pomodoro.new(pomodoroCount);
    counter.counter = tmp.to_time();
  }

  $: if (!isPomodoroActive) {
    let tmp = Pomodoro.new(0);
    counter.counter = tmp.to_pomodoro(timeCount);
    counter.pomodoro = tmp.pomodoro;
  }
</script>

<ConvertCard
  active={isPomodoroActive}
  bind:value={pomodoroCount}
  bind:counter
/>

<button
  class="btn btn-square btn-ghost mx-auto lg:my-auto"
  on:click={() => {
    isPomodoroActive = !isPomodoroActive;
    [pomodoroCount, timeCount] = [timeCount, pomodoroCount];
  }}
>
  <Icon
    src={ArrowsRightLeft}
    theme="solid"
    class="inline-block h-6 stroke-current rotate-90 lg:rotate-0"
  />
</button>

<ConvertCard
  isPomodoro={false}
  active={!isPomodoroActive}
  bind:value={timeCount}
  bind:counter
/>
