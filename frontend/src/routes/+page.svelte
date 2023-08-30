<script lang="ts">
  import ConvertCard from "../components/ConvertCard.svelte";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { ArrowsRightLeft } from "@steeze-ui/heroicons";
  import { Pomodoro, Counter } from "./../wasm/pomolib";
  import type { PomodoroCounter } from "../types";
  import type { TimeValue } from "../components/types";

  let isPomodoroActive = true;

  let pomodoroCount: number;
  let timeCount: TimeValue = {
      hours: 0,
      minutes: 0
  };
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
    counter.counter = tmp.to_pomodoro(timeCount.hours * 60 + timeCount.minutes);
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
    // If we are switching from pomodoro to time
    if (isPomodoroActive) {
      // update the value of the timeCount var to match the value of the previouly set pomodoro numbers
      timeCount.hours = Math.floor((counter.counter.work_time + counter.counter.chill_time) / 60);
      timeCount.minutes = (counter.counter.work_time + counter.counter.chill_time) % 60;
    } else {
      // update the value of the pomodoroCount var to match the corresponding time available
      pomodoroCount = counter.pomodoro;
    }
    isPomodoroActive = !isPomodoroActive;
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
