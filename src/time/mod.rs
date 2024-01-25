use bevy::prelude::{App, Component, Plugin, Resource};
#[allow(unused_imports)]
use bevy::utils::Duration; // TODO
use std::convert::From;

/* WARN / TODO should probably use this since it exists:

https://docs.rs/bevy/latest/bevy/utils/struct.Dur.html

painful to refactor, but .. less so now than later ..
also collisions with Dur and Direction suck, I should just rename them

*/

// at 10 ticks / second, a u32 is enough for 13 years worth of game time.
// use u32 for everything to avoid casting.

#[derive(Default)]
pub(crate) struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickCount(0));
    }
}

// pub(crate) fn advance_global_tick(mut time: ResMut<TickCount>) {
//     time.tick();
// }

// util

#[derive(Copy, Clone, Debug, Component, Eq, PartialEq, Ord, PartialOrd)]
#[allow(dead_code)]
pub(crate) enum Unit {
    Tick = 1,
    Second = 10,
    Minute = 600,
    Hour = 3600,
    Day = 864_000,
    Week = 6_048_000,
    Year = 315_360_000,
}

#[derive(Copy, Clone, Debug, Component, Eq)]
struct Dur {
    unit: Unit,
    value: u32,
}

impl Dur {
    pub fn as_u32(&self) -> u32 {
        Into::<u32>::into(*self)
    }
}

impl PartialEq for Dur {
    fn eq(&self, other: &Dur) -> bool {
        // &(Into::<u32>::into(*self) as u32) == &(Into::<u32>::into(*other) as u32)
        self.as_u32().eq(&other.as_u32())
    }
}
impl PartialOrd for Dur {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_u32().partial_cmp(&other.as_u32())
    }
}

impl Ord for Dur {
    fn cmp(&self, other: &Dur) -> std::cmp::Ordering {
        self.as_u32().cmp(&other.as_u32())
    }
}

// units of time - conversions to u32 (tick)
#[allow(dead_code)]
impl Unit {
    pub fn seconds(seconds: u32) -> u32 {
        Unit::Second as u32 * seconds
    }

    pub fn minutes(minutes: u32) -> u32 {
        Unit::Minute as u32 * minutes
    }

    pub fn hours(hours: u32) -> u32 {
        Unit::Hour as u32 * hours
    }

    pub fn days(days: u32) -> u32 {
        Unit::Day as u32 * days
    }

    pub fn weeks(weeks: u32) -> u32 {
        Unit::Week as u32 * weeks
    }

    pub fn years(years: u32) -> u32 {
        Unit::Year as u32 * years
    }
}

#[derive(Resource, Component, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub(crate) struct TickCount(pub(crate) u32);

#[allow(dead_code)]
impl TickCount {
    pub fn as_clock(&self) -> Clock {
        Clock::new(*self)
    }

    pub fn tick(&mut self) {
        self.advance(1)
    }

    pub fn advance(&mut self, t: u32) {
        self.0 = self.0.checked_add(t).unwrap_or(0);
    }

    // pub fn advance_by(&mut self, duration: Dur) {
    //     self.advance(duration.into());
    // }

    pub fn add(&self, other: u32) -> u32 {
        self.0 + other
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<u32> for TickCount {
    fn from(tick: u32) -> TickCount {
        TickCount(tick)
    }
}

impl From<TickCount> for u32 {
    fn from(time: TickCount) -> u32 {
        time.0
    }
}
impl From<Dur> for u32 {
    fn from(duration: Dur) -> u32 {
        duration.unit as u32 * duration.value
    }
}

#[derive(Resource, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Clock {
    time: TickCount,

    year: u32,
    day_of_year: u32,
    week: u32,
    weekday: u32,
    hour: u32,
    minute: u32,
    second: u32,
    tick: u32, // since last whole second
}

#[allow(dead_code)]
impl Clock {
    pub(crate) fn new(time: TickCount) -> Clock {
        let mut tick = time.0;

        let year = tick / Unit::Year as u32;
        tick -= year * Unit::Year as u32;
        let day_of_year = tick / Unit::Day as u32;

        let week = tick / Unit::Week as u32;
        tick -= week * Unit::Week as u32;

        let weekday = tick / Unit::Day as u32;
        tick -= weekday * Unit::Day as u32;

        let hour = tick / Unit::Hour as u32;
        tick -= hour * Unit::Hour as u32;

        let minute = tick / Unit::Minute as u32;
        tick -= minute * Unit::Minute as u32;

        let second = tick / Unit::Second as u32;
        tick -= second * Unit::Second as u32;

        Clock {
            time: TickCount(tick),
            year,
            day_of_year,
            week,
            weekday,
            hour,
            minute,
            second,
            tick,
        }
    }
}
