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

use action::{PlayerActionInvalidEvent, StillWaitForAnimEvent};
use graphics::LogicalGraphicalEntityMapper;
use player::SpawnPlayerEvent;
use typical::graphics::*;

use bevy::ecs::schedule::common_conditions::*;
pub use bevy::prelude::{
    apply_deferred, default, on_event, state_exists, state_exists_and_equals, First, Has,
    IntoSystemConfigs, Last, OnEnter, OnExit, OnTransition, PostUpdate, PreUpdate, Startup, Update,
};

// use bevy::ecs::schedule::*;

use bevy::prelude::{DefaultPlugins, PluginGroup};
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_turborand::prelude::RngPlugin;

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
        .add_event::<SpawnPlayerEvent>()
        .add_event::<PlayerActionInvalidEvent>()
        .add_event::<StillWaitForAnimEvent>()
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
        // OnEnter(TickState) systems
        .add_systems(
            OnEnter(TickState::PlayerInput),
            (action::skip_player_input_if_action_exists, apply_deferred).chain(),
        )
        .add_systems(
            OnEnter(TickState::ValidatePlayerAction),
            (
                input::validate_player_move,
                // other validations go here
                input::handle_ev_player_action_invalid
                    .run_if(on_event::<action::PlayerActionInvalidEvent>()),
                apply_deferred,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(TickState::PrepareAgentActions),
            (action::prepare_agent_actions, apply_deferred).chain(),
        )
        .add_systems(OnEnter(TickState::ClockTick), action::clock_tick)
        .add_systems(
            OnEnter(TickState::ApplyCompletedActions),
            (action::apply_completed_action_markers, apply_deferred).chain(),
        )
        // PreUpdate
        // validate, apply actions, etc
        .add_systems(
            PreUpdate,
            (
                input::keybindings.run_if(state_exists_and_equals(TickState::PlayerInput)),
                apply_deferred,
            )
                .chain(),
        )
        .add_systems(
            PreUpdate,
            (
                action::tick_player_action
                    .run_if(state_exists_and_equals(TickState::PlayerActionTick)),
                apply_deferred,
            )
                .chain(),
        )
        .add_systems(
            PreUpdate,
            (
                action::tick_agent_actions
                    .run_if(state_exists_and_equals(TickState::AgentActionsTick)),
                apply_deferred,
            )
                .chain(),
        )
        // Update
        // actually apply effects of actions
        .add_systems(
            Update,
            (
                action::on_success::apply_move,
                action::on_success::apply_attack,
                apply_deferred,
            )
                .chain()
                .run_if(state_exists_and_equals(TickState::ApplyCompletedActions)),
        )
        .add_systems(
            Update,
            (graphics::move_anim::lerp_vec3_translation)
                .chain()
                .run_if(state_exists_and_equals(TickState::Animate)),
        )
        // Check if animations are complete, if so GOTO 10
        .add_systems(
            PostUpdate,
            action::dead_letters.run_if(state_exists_and_equals(TickState::Animate)),
        )
        //
        // these alway run, in the background:
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
            Update,
            show_instrumentation.run_if(not(state_exists_and_equals(TickState::PlayerInput))),
        )
        .run();
}

fn show_instrumentation(t: Res<bevy::core::FrameCount>, res: Res<State<TickState>>) {
    warn!("😈 frame incremented: {:?} => {:?}", t.0, res.get());
}
