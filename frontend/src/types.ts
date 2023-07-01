import type { Counter } from "./wasm/pomolib";

export interface PomodoroCounter {
  pomodoro: number;
  counter: Counter;
}
