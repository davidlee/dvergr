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
pub mod time;
pub mod typical;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::window::{PresentMode, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_turborand::prelude::RngPlugin;
use graphics::LogicalGraphicalEntityMapper;
use input::PlayerInputState;
use player::SpawnPlayerEvent;
use typical::graphics::*;

// System sets and such
#[derive(Ord, ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone, PartialOrd)]
enum ActionSchedule {
    // Plan:
    Assign,
    Validate,
    // Run:
    Tick,
    Apply,
    // AwaitAnim:
    Animate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct CustomFlush;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionSet {
    PreUpdate,
    Update,
    PostUpdate,
}

fn main() {
    // let schedule = Schedule::new(ActionSchedule::Assign);
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
        .add_state::<ActionSystemState>()
        .add_state::<PlayerInputState>()
        // EVENTS
        .add_event::<SpawnPlayerEvent>()
        .add_event::<TickEvent>()
        .add_event::<ActionInvalidEvent>()
        .add_event::<ActionPlanRequestEvent>()
        .add_event::<ActionCompleteEvent>()
        .add_event::<ActionVerifyAssignsEvent>()
        .add_event::<ActionStartEvent>()
        .add_event::<ActionAbortEvent>()
        .add_event::<StillWaitForAnimEvent>()
        //
        // SYSTEMS
        //
        // Startup
        .configure_sets(PreUpdate, ActionSet::PreUpdate)
        .configure_sets(Update, ActionSet::Update)
        .configure_sets(PostUpdate, ActionSet::PostUpdate)
        //
        .add_systems(
            Startup,
            (
                board::generator::populate_board,
                player::spawn,
                apply_deferred, // ensure player exists
                graphics::init_map::spawn_voxel_map,
                apply_deferred,
                graphics::player_avatar::spawn,
                action::bootstrap,
            )
                .chain(),
        )
        .add_systems(
            ActionSchedule::Assign,
            (
                action::check_player_plan,
                apply_deferred.in_set(CustomFlush),
                input::keybindings.run_if(in_state(PlayerInputState::Listen)),
                action::plan_agent_actions.run_if(on_event::<ActionPlanRequestEvent>()),
                action::check_all_plans.run_if(on_event::<ActionVerifyAssignsEvent>()),
            )
                .chain()
                .in_set(ActionSet::PreUpdate)
                .run_if(in_state(ActionSystemState::Plan)),
        )
        // hmmm there's devil in the detail of how we ensure we validate
        // optimistically, as little as possible
        .add_systems(
            ActionSchedule::Validate,
            (
                input::validate_move,
                // TODO other validations
                // ...
                input::handle_action_invalid.run_if(on_event::<ActionInvalidEvent>()),
                apply_deferred.in_set(CustomFlush),
                action::set_state_run, // conditions
                apply_deferred.in_set(CustomFlush),
            )
                .chain()
                .in_set(ActionSet::PreUpdate)
                .run_if(in_state(ActionSystemState::Plan)),
        )
        .add_systems(
            ActionSchedule::Tick,
            (
                action::clock_tick,
                action::tick_actions,
                apply_deferred.in_set(CustomFlush),
            )
                .chain()
                .in_set(ActionSet::PreUpdate)
                .run_if(on_event::<TickEvent>()),
        )
        .add_systems(
            ActionSchedule::Apply,
            (
                action::apply_completed_action_markers,
                apply_deferred.in_set(CustomFlush),
                action::on_success::apply_move,
                action::on_success::apply_attack,
                // ...
                action::set_state_await_anim,
                apply_deferred.in_set(CustomFlush),
            )
                .chain()
                .in_set(ActionSet::PreUpdate)
                .run_if(on_event::<ActionCompleteEvent>()),
        )
        .add_systems(
            ActionSchedule::Animate,
            (
                graphics::player_avatar::flicker_torches,
                graphics::move_anim::animate_player_fov,
                graphics::move_anim::lerp_vec3_translation,
                // keep-alive or return to set player input
                (action::set_state_plan, apply_deferred.in_set(CustomFlush))
                    .chain()
                    .run_if(not(on_event::<StillWaitForAnimEvent>())),
            )
                .chain()
                .in_set(ActionSet::Update)
                .run_if(in_state(ActionSystemState::AwaitAnim)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
