use clap::Parser;
mod types;
use crate::types::Args;
use crate::types::Counter;

/// Simple program convert pomodoros to real time
fn main() {
    let args = Args::parse();

    let mut cycle_counter = Counter {
        cycle: 0,
        work_time: 0,
        chill_time: 0,
    };

    for i in 0..args.pomodoro {
        if i != args.pomodoro - 1 {
            if cycle_counter.cycle == args.reset_point - 1 {
                cycle_counter.chill_time += args.long_pause;
                cycle_counter.cycle = 0;
            } else {
                cycle_counter.cycle += 1;
                cycle_counter.chill_time += args.short_pause;
            }
        }
        cycle_counter.work_time += args.time;
    }

    println!(
        "Work time: {} hour(s) and {} minute(s)",
        cycle_counter.work_time / 60,
        cycle_counter.work_time % 60
    );
    println!(
        "Chill time: {} hour(s) and {} minute(s)",
        cycle_counter.chill_time / 60,
        cycle_counter.chill_time % 60
    );
    println!(
        "\nTotal: {} hour(s) and {} minute(s)",
        (cycle_counter.work_time + cycle_counter.chill_time) / 60,
        (cycle_counter.work_time + cycle_counter.chill_time) % 60
    );
}
