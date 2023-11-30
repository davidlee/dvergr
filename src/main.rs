pub mod action;
pub mod anatomy;
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
    pub use crate::board::{
        cell::Cell,
        direction::Direction,
        primitives::{Area3d, Size3d},
        Board, Material, PlayerCellVisibility, Position,
    };
    pub use crate::creature::{species::Species, Attributes, Creature, CreatureSize, Locus};
    pub use crate::player::Player;
    pub use crate::state::{AppInitEvent, AppState};
    pub use crate::time::Clock;
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
    pub use bevy::utils::tracing::{debug, error, info, trace, warn, Level};
}

use bevy::prelude::{ClearColor, Color, DefaultPlugins, ImagePlugin, PluginGroup};
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_pancam::PanCamPlugin;
use bevy_turborand::prelude::RngPlugin;

use bevy::log::LogPlugin;
use bevy::utils::tracing::Level;
use typical::*;

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
            .set(LogPlugin {
                // level: Level::INFO,
                level: Level::TRACE,
                filter: "wgpu=warn,bevy_ecs=info".to_string(),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),)) // no blurry sprites
        .insert_resource(ClearColor(Color::rgb(0.0, 0.05, 0.15)))
        .add_state::<AppState>()
        .add_event::<creature::movement::StartMove>()
        // plugins
        .add_plugins(FpsCounterPlugin)
        .add_plugins(PanCamPlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(time::TimePlugin)
        .add_plugins(board::plugin::BoardPlugin)
        .add_plugins(graphics::asset_loading::AssetLoadingPlugin)
        //
        // INITIALIZATION
        .add_systems(Startup, ui::spawn_camera)
        .add_systems(
            Update,
            graphics::components::spawn_stage.run_if(state_exists_and_equals(AppState::InitStage)),
        )
        .add_systems(OnEnter(AppState::InitUI), ui::spawn_layout)
        .add_systems(
            OnEnter(AppState::InitAssets),
            (
                graphics::tilemap::load_tileset,
                graphics::mobs::load_spritesheet.after(graphics::tilemap::load_tileset),
            ),
        )
        .add_systems(
            PostUpdate,
            graphics::asset_loading::ensure_assets_loaded
                .run_if(state_exists_and_equals(AppState::LoadAssets)),
        )
        .add_systems(
            OnEnter(AppState::InitTileMap),
            graphics::tilemap::spawn_tile_map,
        )
        .add_systems(OnEnter(AppState::InitPlayer), player::spawn_player)
        .add_systems(
            OnEnter(AppState::InitMobs),
            graphics::player_avatar::spawn_player_avatar,
        )
        //
        // MOVEMENT
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
        .add_systems(
            Update,
            graphics::mobs::add_changed_creature_mob_move_anim
                .run_if(state_exists_and_equals(AppState::Game)),
        )
        .add_systems(
            Update,
            (graphics::mobs::mob_movement
                .after(graphics::mobs::add_changed_creature_mob_move_anim))
            .run_if(state_exists_and_equals(AppState::Game)),
        )
        //
        // VISIBILITY
        .add_systems(
            Update,
            (player::visibility::mark_player_visible_cells.after(graphics::mobs::mob_movement))
                .run_if(state_exists_and_equals(AppState::Game)),
        )
        .add_systems(
            Update,
            (graphics::tilemap::update_tiles_for_player_cell_visibility
                .after(player::visibility::mark_player_visible_cells))
            .run_if(state_exists_and_equals(AppState::Game)),
        )
        // MISC
        //
        //
        .add_systems(
            Update,
            graphics::draw_weird_lines.run_if(state_exists_and_equals(AppState::Game)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(PostUpdate, state::handle_app_init_event) // TODO REMOVE AFTER INIT COMPLETE
        .add_systems(PostUpdate, time::clock_frame_tick)
        // EVENTS
        .add_event::<player::movement::DirectionalInput>()
        .add_event::<state::AppInitEvent>()
        // ok, ready?
        .run();
}
