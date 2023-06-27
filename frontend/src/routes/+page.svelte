<script lang="ts">
  import ConvertCard from "../components/ConvertCard.svelte";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { ArrowsRightLeft } from "@steeze-ui/heroicons";

  let isActive = true;
</script>


<ConvertCard isPomodoro={isActive ? true : false} active={true} />

<button
  class="btn btn-square btn-ghost mx-auto lg:my-auto"
  on:click={() => {
    isActive = !isActive;
  }}
>
  <Icon
    src={ArrowsRightLeft}
    theme="solid"
    class="inline-block h-6 stroke-current"
  />
</button>

<ConvertCard isPomodoro={isActive ? false : true} active={false} />
