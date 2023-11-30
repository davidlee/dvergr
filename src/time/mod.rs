use crate::typical::*;
// use bevy::prelude::{App, Plugin, ResMut, Resource};

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
    fn build(&self, app: &mut App) {
        app.insert_resource(Clock::default());
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
    seconds: f64,
    paused: bool,
    current_frame: u32,
    current_turn: u32,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            seconds: 0.,
            paused: true,
            current_frame: 0,
            current_turn: 0,
        }
    }
}

impl Clock {
    const SECONDS_PER_TURN: f64 = 0.1;
    const ONE_SECOND: f64 = 1.0;

    pub fn next_turn(&mut self) {
        self.seconds += Self::SECONDS_PER_TURN;
        self.current_turn += 1;
    }

    pub fn next_second(&mut self) {
        self.seconds += Self::ONE_SECOND;
        self.current_turn += 10;
    }

    pub fn advance_turns(&mut self, turns: u32) {
        self.current_turn = turns;
        self.seconds += turns as f64 * Self::SECONDS_PER_TURN;
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

    pub fn frame_tick(&mut self) {
        trace!("tick, tock ... {:?})", self.current_frame);
        self.current_frame = self.current_frame.checked_add(1).unwrap_or(0);
    }

    pub fn current_turn(&self) -> u32 {
        self.current_turn
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }
}

pub fn clock_frame_tick(mut clock: ResMut<Clock>) {
    clock.frame_tick();
}

// timers
// events

#[allow(dead_code)]
pub struct Duration {
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: f64,
}
