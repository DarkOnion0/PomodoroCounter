use serde::Serialize;

pub struct Pomodoro {
    /// The number of pomodoro
    pub pomodoro: u32,
    /// The time associated to a pomodoro in minutes
    pub time: u32,
    /// The number of pomodoro before the reset happen
    pub reset_point: u8,
    /// The short pause time in minutes
    pub short_pause: u32,
    /// The long pause time in minutes
    pub long_pause: u32,
}
impl Pomodoro {
    /// Convert pomodoro(s) to minutes
    pub fn to_time(&mut self) -> Counter {
        let mut cycle_counter = Counter::new();

        // Itereate over the pomodoro(s)
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
    /// Convert the `available_time` to the corresping amount of pomodoro
    pub fn to_pomodoro(&mut self, mut available_time: i32) {
        let mut counter = 0;

        // Continue while a new pomodoro can be done (the chill time is not included)
        while available_time >= self.time as i32 {
            self.pomodoro += 1;
            available_time -= self.time as i32;

            // Set the counter reset rules and the corresponding chill time
            if counter == self.reset_point - 1 {
                // reset the counter after and add a long pause when the end of the cycle is
                // reached
                counter = 0;
                available_time -= self.long_pause as i32;
            } else {
                // increase the counter and add a short pause
                counter += 1;
                available_time -= self.short_pause as i32;
            }
        }
    }
    /// Create a new instance
    pub fn new(pomodoro: u32) -> Self {
        // Should be init with the same default values as the clap ones
        Pomodoro {
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
        let mut args = Pomodoro::new(5);
        let result = args.to_time();
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
