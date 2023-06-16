use clap::Parser;
use core::Args;

/// Simple program convert pomodoros to real time
fn main() {
    let mut args = Args::parse();
    let cycle_counter = args.convert();

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
