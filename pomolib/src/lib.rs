use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Default)]
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
#[wasm_bindgen]
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
    pub fn to_pomodoro(&mut self, mut available_time: i32) -> Counter {
        let mut counter = Counter::new();
        let mut last_pause = 0;

        // Continue while a new pomodoro can be done (the chill time is not included)
        while available_time >= self.time as i32 {
            self.pomodoro += 1;
            available_time -= self.time as i32;

            // Set the counter reset rules and the corresponding chill time
            if counter.cycle == self.reset_point - 1 {
                // reset the counter after and add a long pause when the end of the cycle is
                // reached
                counter.cycle = 0;
                available_time -= self.long_pause as i32;
                counter.chill_time += self.long_pause;
                last_pause = self.long_pause;
            } else {
                // increase the counter and add a short pause
                counter.cycle += 1;
                available_time -= self.short_pause as i32;
                counter.chill_time += self.short_pause;
                last_pause = self.short_pause;
            }
            counter.work_time += self.time;
        }

        counter.chill_time -= last_pause;
        available_time += last_pause as i32;

        counter.spare_time = available_time as u32;

        counter
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
#[wasm_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct Counter {
    /// The cycle counter, only used to convert from pomodoro to time
    #[serde(skip_serializing)]
    cycle: u8,

    /// The total work time in minute
    pub work_time: u32,
    /// The total chill time in minute
    pub chill_time: u32,
    /// The time that couldn't be fit in a pomodoro
    pub spare_time: u32,
}
#[wasm_bindgen]
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

    #[test]
    fn time_to_pomodoro_exact_time() {
        let mut pomodoro = Pomodoro::new(0);
        pomodoro.to_pomodoro(160);
        assert_eq!(
            pomodoro.pomodoro, 5,
            "The number of available pomodoro's in 160 mins is not correct [expected: 5; got: {}]",
            pomodoro.pomodoro
        );
    }
    #[test]
    fn time_to_pomodoro_in_betwen() {
        let mut pomodoro = Pomodoro::new(0);
        pomodoro.to_pomodoro(159);
        assert_eq!(
            pomodoro.pomodoro, 4,
            "The number of available pomodoro's in 160 mins is not correct [expected: 4; got: {}]",
            pomodoro.pomodoro
        );
    }
    #[test]
    fn time_to_pomodoro_spare_time() {
        let mut pomodoro = Pomodoro::new(0);
        let counter = pomodoro.to_pomodoro(189);
        assert_eq!(
            counter.spare_time, 29,
            "The spare time for 189mins is not correct [expected: 29; got: {}]",
            pomodoro.pomodoro
        );
    }
}
