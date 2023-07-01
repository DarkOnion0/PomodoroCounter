use serde::{Deserialize, Serialize};
use pomolib::Counter;

#[derive(Serialize)]
pub struct PomodoroCounter {
    pub pomodoro: u32,
    pub counter: Counter,
}

// Should follow the type defition of Pomodoro (must exclude the pomodoro field)
#[derive(Deserialize)]
pub struct RequestParams {
    /// The time associated to a pomodoro in minutes
    pub time: u32,
    /// The number of pomodoro before the reset happen
    pub reset_point: u8,
    /// The short pause time in minutes
    pub short_pause: u32,
    /// The long pause time in minutes
    pub long_pause: u32,
}
impl Default for RequestParams {
    fn default() -> Self {
        Self {
            time: 25,
            reset_point: 4,
            short_pause: 5,
            long_pause: 20,
        }
    }
}
