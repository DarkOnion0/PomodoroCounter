use clap::Parser;

/// Global app config
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The number of pomodoro
    #[arg(short, long)]
    pub pomodoro: u32,
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
impl Args {
    pub fn convert(&mut self) -> Counter {
        let mut cycle_counter = Counter {
            cycle: 0,
            work_time: 0,
            chill_time: 0,
        };

        for i in 0..self.pomodoro {
            if i != self.pomodoro - 1 {
                if cycle_counter.cycle == self.reset_point - 1 {
                    cycle_counter.chill_time += self.long_pause;
                    cycle_counter.cycle = 0;
                } else {
                    cycle_counter.cycle += 1;
                    cycle_counter.chill_time += self.short_pause;
                }
            }
            cycle_counter.work_time += self.time;
        }

        cycle_counter
    }
}

/// The counter status for the requested pomodoro time
pub struct Counter {
    pub cycle: u8,

    pub work_time: u32,
    pub chill_time: u32,
}
