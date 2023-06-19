use clap::{Parser, Subcommand};
use core::Pomodoro;

#[derive(Parser, Debug)]
#[command(author = "DarkOnion0", version, about = "A small CLI utils to convert pomodoros/times", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// The time associated to a pomodoro in minutes
    #[arg(short, long, default_value_t = 25)]
    pub time: u32,
    /// The number of pomodoro before the reset happen
    #[arg(short, long, default_value_t = 4)]
    pub reset_point: u8,
    /// The short pause time in minutes
    #[arg(short, long, default_value_t = 5)]
    pub short_pause: u32,
    /// The long pause time in minutes
    #[arg(short, long, default_value_t = 20)]
    pub long_pause: u32,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert pomodoro(s) to time
    Pomodoro {
        /// The number of pomodoro to convert
        count: u32,
    },
    /// Convert time to pomodoro(s)
    Time {
        /// The number of minutes to convert
        count: u32,
    },
}

/// Simple program convert pomodoros to real time
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Pomodoro { count }) => {
            let mut pomodoro = Pomodoro {
                pomodoro: *count,
                time: cli.time,
                long_pause: cli.long_pause,
                reset_point: cli.reset_point,
                short_pause: cli.short_pause,
            };
            let cycle_counter = pomodoro.to_time();

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
        Some(Commands::Time { count }) => {
            let mut pomodoro = Pomodoro {
                pomodoro: 0,
                time: cli.time,
                long_pause: cli.long_pause,
                reset_point: cli.reset_point,
                short_pause: cli.short_pause,
            };
            let cycle_counter = pomodoro.to_pomodoro(*count as i32);

            println!("Number of pomodoro(s): {}", pomodoro.pomodoro);
            println!("Spare time: {} min(s)", cycle_counter.spare_time);

            println!(
                "\nWork time: {} hour(s) and {} minute(s)",
                cycle_counter.work_time / 60,
                cycle_counter.work_time % 60
            );
            println!(
                "Chill time: {} hour(s) and {} minute(s)",
                cycle_counter.chill_time / 60,
                cycle_counter.chill_time % 60
            );
        }
        None => {}
    }
}
