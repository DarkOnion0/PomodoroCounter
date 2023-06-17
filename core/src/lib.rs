use clap::Parser;
use serde::Serialize;

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
    /// Convert pomodoro(s) to minutes
    pub fn convert(&mut self) -> Counter {
        let mut cycle_counter = Counter::new();

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
    /// Create a new instance of Args
    pub fn new(pomodoro: u32) -> Self {
        // Should be init with the same default values as the clap ones
        Args {
            pomodoro: pomodoro,
            time: 25,
            reset_point: 4,
            short_pause: 5,
            long_pause: 20,
        }
    }
}

/// The counter status for the requested pomodoro time
#[derive(Default, Serialize)]
pub struct Counter {
    /// The cycle counter, only used to convert from pomodoro to time
    #[serde(skip_serializing)]
    cycle: u8,

    /// The total work time in minute
    pub work_time: u32,
    /// The total chill time in minute
    pub chill_time: u32,
}
impl Counter {
    /// Create a new instance of Counter
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Check the validity of the conversion process from pomodoro to time
    fn pomdoro_to_time() {
        let mut args = Args::new(5);
        let result = args.convert();
        assert_eq!(
            result.work_time, 125,
            "The work time for 5 pomodoros is not correct"
        );
        assert_eq!(
            result.chill_time, 35,
            "The chill time for 5 pomodoros is not correct"
        );
    }
}
