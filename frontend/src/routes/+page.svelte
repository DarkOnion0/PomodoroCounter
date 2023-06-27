<script lang="ts">
  import ConvertCard from "../components/ConvertCard.svelte";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { ArrowsRightLeft } from "@steeze-ui/heroicons";

  let isActive = true;

  let pomodoroCount: number;
  let timeCount: number;
</script>


<ConvertCard active={isActive} bind:value={pomodoroCount} />

<button
  class="btn btn-square btn-ghost mx-auto lg:my-auto"
  on:click={() => {
    isActive = !isActive;
  }}
>
  <Icon
    src={ArrowsRightLeft}
    theme="solid"
    class="inline-block h-6 stroke-current rotate-90 lg:rotate-0"
  />
</button>

<ConvertCard isPomodoro={false} active={!isActive} bind:value={timeCount} />
