#![allow(unused_imports)]

// pub(crate) use crate::action;
// pub(crate) use crate::combat;
// pub(crate) use crate::inventory;
// pub(crate) use crate::material;
// pub(crate) use crate::player;
// pub(crate) use crate::state;

pub(crate) use crate::board::{
    cell::{Cell, Floor, Wall},
    direction::{Direction, COMPASS_DEGREES},
    // primitives::{Area3d, Size3d},
    Board,
    BOARD_SIZE_X,
    BOARD_SIZE_Y,
};
pub(crate) use crate::creature::anatomy::humanoid::Location;
pub(crate) use crate::creature::{
    APSymmetry, AbilityList, Creature, CreatureBundle, CreatureSize, Gender, Locus, NeedList, Pace,
    Side, SkillList, Stance,
};
pub(crate) use crate::dice::Dice;
pub(crate) use crate::material::{Material, Species};
pub(crate) use crate::player::Player;
pub(crate) use crate::state::{AppState, TickState};
pub(crate) use crate::time::{Clock, Duration, TickCount, Unit};

pub use bevy::prelude::{
    App, BuildChildren, Bundle, Changed, Commands, Component, Deref, DerefMut, Entity, Event,
    EventReader, EventWriter, NextState, Plugin, Query, Res, ResMut, Resource, State, Transform,
    Vec2, Vec3, With, Without,
};

pub use bevy::core_pipeline::clear_color::ClearColorConfig;
pub use bevy::math::{IVec2, IVec3};
pub use bevy::utils::tracing::*;
pub use bevy::utils::{HashMap, HashSet};

pub use bevy::log::LogPlugin;
pub use bevy::utils::tracing::Level;

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
    pub(crate) use crate::graphics::{
        CameraMarker, CreatureEntityRef, DwarfSpritesheet, TorchMarker, TorchSecondaryLightMarker,
    };
    // these are slightly more public than the above ..
    pub(crate) use crate::graphics::{LogicalGraphicalEntityMapper, MapMarker, PlayerAvatar};

    pub use bevy::prelude::{
        AssetServer, Assets, ClearColor, Color, Handle, Image, Msaa, SpatialBundle, Sprite,
        SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite,
        Transform,
    };
    pub use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
}
