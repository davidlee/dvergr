// use std::fmt::{format, Arguments};h
use std::convert::From;

// use crate::typical::*;
use bevy::prelude::{App, Component, Plugin, ResMut, Resource};

// at 10 ticks / second, a u32 is enough for 13 years worth of game time.
// use u32 for everything to avoid casting.

pub const TICKS_PER_SECOND: u32 = 10;
pub const SECONDS_PER_MINUTE: u32 = 60;
pub const SECONDS_PER_HOUR: u32 = 3600;
pub const SECONDS_PER_DAY: u32 = 86_400;
pub const SECONDS_PER_WEEK: u32 = 604_800;
pub const SECONDS_PER_YEAR: u32 = 31_536_000;

#[derive(Default)]
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Clock::default());
    }
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Duration {
    Ticks(u32), // 100ms
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Weeks(u32),
    Years(u32),
}
impl Default for Duration {
    fn default() -> Self {
        Duration::Ticks(0)
    }
}

impl Duration {
    pub fn as_ticks(&self) -> Duration {
        Duration::Ticks(u32::from(*self))
    }
    pub fn as_seconds(&self) -> Duration {
        Duration::Seconds(u32::from(*self) / TICKS_PER_SECOND)
    }
    pub fn as_minutes(&self) -> Duration {
        Duration::Minutes(u32::from(*self) / TICKS_PER_SECOND / SECONDS_PER_MINUTE)
    }
    pub fn as_hours(&self) -> Duration {
        Duration::Hours(u32::from(*self) / TICKS_PER_SECOND / SECONDS_PER_HOUR)
    }
    pub fn as_days(&self) -> Duration {
        Duration::Days(u32::from(*self) / TICKS_PER_SECOND / SECONDS_PER_DAY)
    }
    pub fn as_weeks(&self) -> Duration {
        Duration::Weeks(u32::from(*self) / TICKS_PER_SECOND / SECONDS_PER_WEEK)
    }
    pub fn as_years(&self) -> Duration {
        Duration::Years(u32::from(*self) / TICKS_PER_SECOND / SECONDS_PER_YEAR)
    }
}

impl From<Duration> for u32 {
    fn from(duration: Duration) -> u32 {
        match duration {
            Duration::Ticks(ticks) => ticks,
            Duration::Seconds(seconds) => seconds * TICKS_PER_SECOND,
            Duration::Minutes(minutes) => minutes * SECONDS_PER_MINUTE * TICKS_PER_SECOND,
            Duration::Hours(hours) => hours * SECONDS_PER_HOUR * TICKS_PER_SECOND,
            Duration::Days(days) => days * SECONDS_PER_DAY * TICKS_PER_SECOND,
            Duration::Weeks(weeks) => weeks * SECONDS_PER_WEEK * TICKS_PER_SECOND,
            Duration::Years(years) => years * SECONDS_PER_YEAR * TICKS_PER_SECOND,
        }
    }
}

impl From<u32> for Duration {
    fn from(tick: u32) -> Duration {
        Duration::Ticks(tick)
    }
}

#[derive(Resource, Debug, Default)]
pub struct Clock {
    tick: u32,
    second: u32,
    minute: u32,
    hour: u32,
    weekday: u32,
    week: u32,
    day_of_year: u32,
    year: u32,
}

impl Clock {
    pub fn tock(&mut self) {
        self.tick = self.tick.checked_add(1).unwrap_or(0);
        match self.tick {
            _ if self.tick % (SECONDS_PER_YEAR * TICKS_PER_SECOND) == 0 => {
                self.year += 1;
                self.week = 0;
                self.weekday = 0;
                self.day_of_year = 0;
                self.hour = 0;
                self.minute = 0;
                self.second = 0;
                // event
            }
            _ if self.tick % (SECONDS_PER_WEEK * TICKS_PER_SECOND) == 0 => {
                self.week += 1;
                self.weekday = 0;
                self.hour = 0;
                self.minute = 0;
                self.second = 0;
                // event
            }
            _ if self.tick % (SECONDS_PER_DAY * TICKS_PER_SECOND) == 0 => {
                self.weekday += 1;
                self.day_of_year += 1;
                self.hour = 0;
                self.minute = 0;
                self.second = 0;
                // event
            }
            _ if self.tick % (SECONDS_PER_HOUR * TICKS_PER_SECOND) == 0 => {
                self.hour += 1;
                self.minute = 0;
                self.second = 0;
                // event
            }
            _ if self.tick % (SECONDS_PER_MINUTE * TICKS_PER_SECOND) == 0 => {
                self.minute += 1;
                self.second = 0;
                // event
            }
            _ if self.tick % TICKS_PER_SECOND == 0 => {
                self.second += 1;
                dbg!("second", self);
                // event
            }
            _ => (),
        }
    }

    pub fn advance_by(&mut self, duration: Duration) {
        for _ in 0..u32::from(duration) {
            self.tock();
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Time: {:?}:{:?}:{:?} :: Turn :: [{:?}]\n",
            self.hour, self.minute, self.second, self.tick,
        )
    }
}

pub fn clock_frame_tick(
    mut clock: ResMut<Clock>,
    // commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut ui_query: Query<(&mut Text, &UIConsole)>,
) {
    clock.tock();

    // if clock.current_frame % 100 == 0 {
    //     if let Ok((mut text, _console)) = ui_query.get_single_mut() {
    //         text.sections.push(mk_console_time_text_section(
    //             clock.as_ref(),
    //             asset_server.load("font/BigBlueTerminalPlus.ttf"),
    //         ));
    //     }
    // }
}

// fn mk_console_time_text_section(clock: &Clock, font: Handle<Font>) -> TextSection {
//     // update something
//     let text_style = TextStyle {
//         font,
//         font_size: 11.0,
//         color: Color::rgb(0.9, 0.9, 0.9),
//     };
//     let mut section = TextSection::from_style(text_style);
//     section.value = clock.display();
//     section
// }

// timers
// events

// pub struct DurationModulo {
//     pub days: u32,
//     pub hours: u32,
//     pub minutes: u32,
//     pub seconds: u32,
//     pub milliseconds: u32,
// }
