use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of pomodoro
    #[arg(short, long)]
    pomodoro: u32,
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
        chill_time: 0
    };

    for _ in 0..args.pomodoro {
        if cycle_counter.cycle == 3 {
            cycle_counter.chill_time += 20;
            cycle_counter.cycle = 0;
        } else {
            cycle_counter.cycle += 1;
            cycle_counter.chill_time += 5;
        }
        cycle_counter.work_time += 25;
    }

    println!("Work time: {} hour(s) and {} minute(s)", cycle_counter.work_time / 60, cycle_counter.work_time % 60);
    println!("Chill time: {} hour(s) and {} minute(s)", cycle_counter.chill_time / 60, cycle_counter.chill_time % 60 );
    println!("\nTotal: {} hour(s) and {} minute(s)", (cycle_counter.work_time + cycle_counter.chill_time) / 60,(cycle_counter.work_time + cycle_counter.chill_time) % 60);
}
