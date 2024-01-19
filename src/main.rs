// #![allow(dead_code)]

pub mod action;
pub mod board;
pub mod combat;
pub mod creature;
pub mod dice;
pub mod graphics;
pub mod input;
pub mod inventory;
pub mod material;
pub mod player;
pub mod state;
pub mod time;

pub mod typical;

use bevy::prelude::{ClearColor, Color, DefaultPlugins, PluginGroup};
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_turborand::prelude::RngPlugin;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::tracing::Level;
use graphics::LogicalGraphicalEntityMapper;
use player::SpawnPlayerEvent;
use state::TickState;
use typical::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
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
                    level: Level::INFO,
                    filter: "wgpu=warn,bevy_ecs=info".to_string(),
                }),
        )
        // PLUGINS
        .add_plugins(FpsCounterPlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(time::TimePlugin)
        // RESOURCES
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LogicalGraphicalEntityMapper::new())
        .insert_resource(Msaa::default())
        .init_resource::<Board>()
        // STATE
        .add_state::<AppState>()
        .add_state::<TickState>()
        // EVENTS
        // .add_event::<input::UpdateLocus>()
        .add_event::<state::AppInitEvent>()
        .add_event::<player::SpawnPlayerEvent>()
        .add_event::<action::PlayerActionInvalidEvent>()
        .add_event::<action::StillWaitForAnimEvent>()
        //
        // SYSTEMS
        //
        // Startup
        .add_systems(
            Startup,
            (
                board::generator::populate_board,
                player::spawn,
                apply_deferred, // ensure player exists
                graphics::init_map::spawn_voxel_map,
                apply_deferred,
                graphics::player_avatar::spawn,
            )
                .chain(),
        )
        // Actions
        .add_systems(
            OnEnter(TickState::PlayerInput),
            action::skip_player_input_if_action_exists,
        )
        .add_systems(
            OnEnter(TickState::ValidatePlayerAction),
            input::validate_player_move,
        )
        .add_systems(
            OnEnter(TickState::PrepareAgentActions),
            action::prepare_agent_actions,
        )
        .add_systems(OnEnter(TickState::ClockTick), action::clock_tick)
        .add_systems(
            OnEnter(TickState::ApplyCompletedActions),
            action::apply_completed_action_markers,
        )
        .add_systems(
            PreUpdate,
            input::keybindings.run_if(state_exists_and_equals(TickState::PlayerInput)),
        )
        .add_systems(
            PreUpdate,
            action::tick_player_action.run_if(state_exists_and_equals(TickState::PlayerActionTick)),
        )
        .add_systems(
            PreUpdate,
            action::tick_agent_actions.run_if(state_exists_and_equals(TickState::AgentActionsTick)),
        )
        // actually apply effects
        .add_systems(
            Update,
            (
                action::on_success::apply_move,
                action::on_success::apply_attack,
            )
                .chain()
                .run_if(state_exists_and_equals(TickState::ApplyCompletedActions)),
        )
        .add_systems(
            PostUpdate,
            action::dead_letters.run_if(state_exists_and_equals(TickState::Animate)),
        )
        .add_systems(
            Update,
            (graphics::move_anim::lerp_vec3_translation)
                .chain()
                .run_if(state_exists_and_equals(TickState::Animate)),
        )
        .add_systems(
            Update,
            (
                graphics::player_avatar::flicker_torches,
                graphics::move_anim::animate_player_fov,
            )
                .run_if(state_exists_and_equals(AppState::Ready)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            PostUpdate,
            state::handle_app_init_event.run_if(on_event::<AppInitEvent>()),
        )
        .run();
}
