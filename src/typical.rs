#![allow(unused_imports)]

pub(crate) use crate::action::{events::*, ActionSystemState, Actor};
pub(crate) use crate::goblin::SpawnGoblinEvent;
pub(crate) use crate::marker_components::*;
// pub(crate) use crate::combat;
// pub(crate) use crate::inventory;
// pub(crate) use crate::material;
// pub(crate) use crate::player;
// pub(crate) use crate::state;

pub(crate) use crate::board::{
    cell::{Cell, Floor, Wall},
    direction::{Dir, COMPASS_DEGREES},
    // primitives::{Area3d, Size3d},
    Board,
    BOARD_SIZE_X,
    BOARD_SIZE_Y,
};
pub(crate) use crate::creature::anatomy::humanoid::Location;
pub(crate) use crate::creature::{
    APSymmetry, AbilityList, Character, CharacterBundle, CharacterLevel, Creature, CreatureBundle,
    CreatureSize, Gender, Locus, NeedList, Pace, Side, SkillList, Stance,
};
pub(crate) use crate::dice::Dice;
pub(crate) use crate::material::{Species, Substance};
pub(crate) use crate::player::Player;
pub(crate) use crate::time::{Clock, TickCount, Unit};

pub use bevy::core::{FrameCount, FrameCountPlugin};
pub use bevy::core_pipeline::clear_color::ClearColorConfig;
pub use bevy::log::LogPlugin;
pub use bevy::math::{IVec2, IVec3};
pub use bevy::prelude::*;
pub use bevy::utils::tracing::Level;
pub use bevy::utils::{HashMap, HashSet};

pub use bevy_turborand::prelude::*;
pub use bevy_turborand::GlobalChaChaRng;
pub use bevy_turborand::RngComponent;

pub use std::cmp::Ordering;
pub use std::collections::VecDeque;
pub use std::convert::From;
pub use std::fmt::Debug;
pub use std::hash::Hash;

pub(crate) mod graphics {
    pub(crate) use super::*;
    pub(crate) use crate::graphics::anim::LerpVec3;
    pub(crate) use crate::graphics::*;

    pub use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
}
