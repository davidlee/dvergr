use bevy::prelude::{App, Plugin, Resource};

pub mod f64 {
    pub const SECONDS_PER_MINUTE: f64 = 60.0;
    pub const SECONDS_PER_HOUR: f64 = 3600.0;
    pub const SECONDS_PER_DAY: f64 = 86400.0;
}
pub mod u32 {
    pub const SECONDS_PER_MINUTE: u32 = 60;
    pub const SECONDS_PER_HOUR: u32 = 3600;
    pub const SECONDS_PER_DAY: u32 = 86400;
}

#[derive(Default)]
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, _app: &mut App) {
        //
    }
}

pub struct Seconds;

impl Seconds {
    pub fn to_minutes(seconds: f64) -> f64 {
        seconds / f64::SECONDS_PER_MINUTE
    }

    pub fn to_hours(seconds: f64) -> f64 {
        seconds / f64::SECONDS_PER_HOUR
    }

    pub fn to_days(seconds: f64) -> f64 {
        seconds / f64::SECONDS_PER_DAY
    }

    pub fn to_duration(seconds: f64) -> Duration {
        let days: f64 = Seconds::to_days(seconds);
        let hours: f64 = Seconds::to_hours(seconds);
        let minutes: f64 = Seconds::to_minutes(seconds);

        let seconds_rem = seconds
            - days * f64::SECONDS_PER_DAY
            - hours * f64::SECONDS_PER_HOUR
            - minutes * f64::SECONDS_PER_MINUTE;

        Duration {
            days: days as u32,
            hours: hours as u32,
            minutes: minutes as u32,
            seconds: seconds_rem,
        }
    }
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct Clock {
    // turn: u32,
    seconds: f64,
    paused: bool,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            seconds: 0.,
            paused: true,
        }
    }
}

impl Clock {
    pub fn next_frame(&mut self) {
        self.seconds += 0.1;
    }

    pub fn next_second(&mut self) {
        self.seconds += 1.;
    }

    pub fn duration(&self) -> Duration {
        Seconds::to_duration(self.seconds)
    }

    pub fn minutes(&self) -> f64 {
        Seconds::to_minutes(self.seconds)
    }

    pub fn hours(&self) -> f64 {
        Seconds::to_hours(self.seconds)
    }

    pub fn days(&self) -> f64 {
        Seconds::to_days(self.seconds)
    }
}
// timers
// events

impl Plugin for Clock {
    fn build(&self, _app: &mut App) {}
}

#[allow(dead_code)]
pub struct Duration {
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: f64,
}
