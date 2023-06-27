<script lang="ts">
  import "../app.postcss";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { Github } from "@steeze-ui/simple-icons";
  import { Cog6Tooth } from "@steeze-ui/heroicons";
</script>

<div class="flex h-screen w-screen overflow-hidden flex-col">
  <!--
  NavBar
  -->
  <div class="navbar bg-base-100">
    <div class="flex-1">
      <a href="/" class="btn btn-ghost normal-case text-xl">PomodoroCounter</a>
    </div>

    <div class="flex-none">
      <a
        href="https://github.com/DarkOnion0/PomodoroCounter"
        class="btn btn-square btn-ghost"
      >
        <Icon
          src={Github}
          theme="solid"
          class="inline-block h-6 stroke-current"
        />
      </a>
      <button class="btn btn-square btn-ghost" onclick="my_modal_2.showModal()">
        <Icon
          src={Cog6Tooth}
          theme="solid"
          class="inline-block h-7 stroke-current"
        />
      </button>
    </div>
  </div>

  <!--
  Body
  -->
  <div class="flex flex-col lg:flex-row w-full h-full justify-evenly">
    <slot />
  </div>
</div>

<!--
Background
-->
<dialog id="my_modal_2" class="modal modal-bottom sm:modal-middle">
  <form method="dialog" class="modal-box">
    <h3 class="font-bold text-lg">Settings</h3>
    <p class="py-4">Press ESC key or click outside to close</p>
  </form>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>
