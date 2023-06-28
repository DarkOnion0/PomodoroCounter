<script lang="ts">
  import ConvertCard from "../components/ConvertCard.svelte";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { ArrowsRightLeft } from "@steeze-ui/heroicons";

  let isPomodoroActive = true;

  let pomodoroCount: number;
  let timeCount: number;
</script>

<ConvertCard active={isPomodoroActive} bind:value={pomodoroCount} />

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
/>
