pub mod action;
pub mod anatomy;
pub mod attributes;
pub mod board;
pub mod creature;
pub mod damage;
pub mod dice;
pub mod graphics;
pub mod input;
pub mod player;
pub mod state;
pub mod time;
pub mod ui;

pub mod typical {
    pub use crate::attributes::Attributes;
    pub use crate::board::{Board, Cell, Direction, PlayerCellVisibility, Position};
    pub use crate::creature::{Creature, Locus, Species};
    pub use crate::player::Player;
    pub use crate::state::AppState;
    pub use bevy::math::{IVec2, IVec3, UVec2, UVec3};
    pub use bevy::prelude::{
        default, on_event, state_exists, state_exists_and_equals, App, BuildChildren, Bundle,
        Changed, Commands, Component, Deref, DerefMut, Entity, Event, EventReader, EventWriter,
        Has, IntoSystemConfigs, NextState, Plugin, Query, Res, ResMut, Resource, State, Vec2, Vec3,
        With, Without,
    };
    pub use bevy::prelude::{
        First, Last, OnEnter, OnExit, OnTransition, PostUpdate, PreUpdate, Startup, Update,
    };
}

use typical::*;

use bevy::prelude::{DefaultPlugins, ImagePlugin, PluginGroup};
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
use bevy_pancam::PanCamPlugin;
use bevy_turborand::prelude::RngPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "~= D V E R G R =~".into(),
                    resolution: WindowResolution::new(2800.0, 1400.0),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),)) // no blurry sprites
        .add_state::<AppState>()
        .add_event::<creature::movement::StartMove>()
        // plugins
        .add_plugins(PanCamPlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(time::TimePlugin)
        .add_plugins(board::BoardPlugin)
        .add_plugins(graphics::AssetLoadingPlugin)
        // During init
        .add_systems(
            Update,
            graphics::spawn_stage.run_if(state_exists_and_equals(AppState::InitStage)),
        )
        .add_systems(
            OnEnter(AppState::InitTileMap),
            graphics::tilemap::spawn_tile_map,
        )
        .add_systems(OnEnter(AppState::InitPlayer), player::spawn_player_bundle)
        .add_systems(
            OnEnter(AppState::InitMobs),
            graphics::mobs::spawn_player_sprite,
        )
        // During run loop
        .add_systems(Update, graphics::mobs::add_changed_creature_mob_move_anim)
        .add_systems(
            Update,
            graphics::mobs::mob_movement.after(graphics::mobs::add_changed_creature_mob_move_anim),
        )
        .add_systems(
            Update,
            player::visibility::mark_player_visible_cells.after(graphics::mobs::mob_movement),
        )
        .add_systems(
            Update,
            graphics::tilemap::update_tiles_for_player_cell_visibility
                .after(player::visibility::mark_player_visible_cells),
        )
        // Movement
        .add_systems(
            PreUpdate,
            input::keybindings.run_if(state_exists_and_equals(AppState::Game)),
        )
        .add_systems(
            PreUpdate,
            (player::movement::validate_directional_input.after(input::keybindings))
                .run_if(state_exists_and_equals(AppState::Game)),
        )
        .add_systems(
            PreUpdate,
            (creature::movement::process_movement
                .after(player::movement::validate_directional_input))
            .run_if(state_exists_and_equals(AppState::Game)),
        )
        .add_event::<player::movement::DirectionalInput>()
        // systems
        .add_systems(Startup, ui::spawn_camera)
        // .add_systems(OnEnter(AppState::InitUI), ui::spawn_layout)
        .add_systems(OnEnter(AppState::InitUI), ui::spawn_layout_shim)
        .add_systems(Update, bevy::window::close_on_esc)
        // events
        .run();
}
