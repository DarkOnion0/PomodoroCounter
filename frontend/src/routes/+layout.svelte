<script lang="ts">
  import "../app.postcss";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { Github } from "@steeze-ui/simple-icons";
  import { Cog6Tooth } from "@steeze-ui/heroicons";

  let pomodoroResetPoint = +localStorage.getItem("pomodoroResetPoint") || 4;
  let pomodoroTime = +localStorage.getItem("pomodoroTime") || 25;
  let pomodoroShortPause = +localStorage.getItem("pomodoroShortPause") || 5;
  let pomodoroLongPause = +localStorage.getItem("pomodoroLongPause") || 20;

  $: localStorage.setItem("pomodoroResetPoint", pomodoroResetPoint.toString());
  $: localStorage.setItem("pomodoroTime", pomodoroTime.toString());
  $: localStorage.setItem("pomodoroShortPause", pomodoroShortPause.toString());
  $: localStorage.setItem("pomodoroLongPause", pomodoroLongPause.toString());

  function resetParams() {
    pomodoroResetPoint = 4;
    pomodoroTime = 25;
    pomodoroShortPause = 5;
    pomodoroLongPause = 20;
  }
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
        target="_blank"
        href="https://github.com/DarkOnion0/PomodoroCounter"
        class="btn btn-square btn-ghost"
      >
        <Icon
          src={Github}
          theme="solid"
          class="inline-block h-6 stroke-current"
        />
      </a>
      <button class="btn btn-square btn-ghost" onclick="settings.showModal()">
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
  <div class="flex flex-col lg:flex-row w-full h-full justify-evenly items-center">
    <slot />
  </div>
</div>

<!--
Background
-->
<dialog id="settings" class="modal modal-bottom sm:modal-middle">
  <form method="dialog" class="modal-box">
    <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
      >✕
    </button>

    <h3 class="font-bold text-lg">Settings</h3>

    <div>
      <label class="label">
        <span class="label-text">Time break</span>
      </label>
      <input
        placeholder="Enter a number of round"
        type="number"
        class="input input-bordered w-full"
        bind:value={pomodoroResetPoint}
      />
    </div>

    <div>
      <label class="label">
        <span class="label-text">Time associated with a pomodoro</span>
      </label>
      <input
        placeholder="Enter some minutes"
        type="number"
        class="input input-bordered w-full"
        bind:value={pomodoroTime}
      />
    </div>

    <div>
      <label class="label">
        <span class="label-text">Time associated with a short pause</span>
      </label>
      <input
        placeholder="Enter some minutes"
        type="number"
        class="input input-bordered w-full"
        bind:value={pomodoroShortPause}
      />
    </div>

    <div>
      <label class="label">
        <span class="label-text">Time associated with a short pause</span>
      </label>
      <input
        placeholder="Enter some minutes"
        type="number"
        class="input input-bordered w-full"
        bind:value={pomodoroLongPause}
      />
    </div>
    <div class="modal-action">
      <button class="btn btn-warning" on:click={resetParams}>Reset</button>
      <button class="btn">Close</button>
    </div>
  </form>
</dialog>
