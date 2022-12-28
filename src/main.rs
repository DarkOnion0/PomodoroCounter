use clap::Parser;

/// Simple program convert pomodoros to real time
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of pomodoro
    #[arg(short, long)]
    pomodoro: u32,
    /// The time associated to a pomodoro in minutes
    #[arg(short, long, default_value_t = 25)]
    time: u32,
    /// The number of pomodoro before the reset happen
    #[arg(short, long, default_value_t = 4)]
    reset_point: u8,
    /// The short pause time in minutes
    #[arg(short, long, default_value_t = 5)]
    short_pause: u32,
    /// The long pause time in minutes
    #[arg(short, long, default_value_t = 20)]
    long_pause: u32,
}

struct Counter {
    cycle: u8,

    work_time: u32,
    chill_time: u32,
}

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
