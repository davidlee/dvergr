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

use bevy::window::{PresentMode, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_turborand::prelude::RngPlugin;
use graphics::LogicalGraphicalEntityMapper;
use input::PlayerInputState;
use player::SpawnPlayerEvent;
use typical::graphics::*;

// System sets and such
#[derive(Ord, SystemSet, Debug, Hash, PartialEq, Eq, Clone, PartialOrd)]
enum ActionSet {
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
struct ActorBehaviour;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// enum ActionSet {
//     PreUpdate,
//     Update,
//     PostUpdate,
// }

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
        .add_event::<ActionInvalidatedEvent>()
        .add_event::<ActionValidatedEvent>()
        .add_event::<ActionPlanRequestEvent>()
        .add_event::<ActionCompleteEvent>()
        .add_event::<ActionAddedEvent>()
        .add_event::<ActionStartedEvent>()
        .add_event::<ActionAbortedEvent>()
        .add_event::<StillWaitForAnimEvent>()
        //
        // SYSTEMS
        //
        // Startup
        // .configure_sets(PreUpdate, ActionSet::PreUpdate)
        // .configure_sets(Update, ActionSet::Update)
        // .configure_sets(PostUpdate, ActionSet::PostUpdate)
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
        // Custom schedules: how do they even work?
        // here we're configuring our custom SystemSet
        //
        // .configure_sets(PreUpdate, ActionSet::PreUpdate)
        // .configure_sets(Update, ActionSet::Update)
        // .configure_sets(PostUpdate, ActionSet::PostUpdate)
        // .configure_sets(PreUpdate, ActorBehaviour)
        // .configure_sets(Update, ActionSet::Update)
        // .configure_sets(PostUpdate, ActionSet::PostUpdate)
        .configure_sets(
            PreUpdate,
            (
                ActionSet::Assign,
                ActionSet::Validate,
                ActionSet::Tick,
                ActionSet::Apply,
                ActionSet::Animate,
            )
                .chain(),
        )
        //
        .add_systems(OnEnter(ActionSystemState::Plan), action::init_or_check_plan)
        .add_systems(
            PreUpdate,
            (
                (
                    // should this run first or last?
                    action::init_or_check_plan.run_if(
                        on_event::<ActionAddedEvent>().or_else(on_event::<ActionValidatedEvent>()),
                    ),
                    input::keybindings.run_if(in_state(PlayerInputState::Listen)),
                    action::plan_agent_actions.run_if(on_event::<ActionPlanRequestEvent>()),
                    apply_deferred.run_if(on_event::<ActionAddedEvent>()),
                )
                    .chain()
                    .in_set(ActionSet::Assign)
                    .run_if(in_state(ActionSystemState::Plan)),
                (
                    action::validation::validate_move.run_if(on_event::<ActionAddedEvent>()),
                    // put more validations here

                    // ensure ActionAddedEvent is consumed

                    // then ..
                    action::handle_action_invalid.run_if(on_event::<ActionInvalidatedEvent>()),
                    apply_deferred,
                )
                    .chain()
                    .in_set(ActionSet::Validate)
                    .run_if(in_state(ActionSystemState::Plan)),
                (
                    action::set_state_run,
                    action::clock_tick,
                    action::tick_actions,
                    // now check if ready for tick
                    apply_deferred,
                )
                    .chain()
                    .in_set(ActionSet::Tick)
                    .run_if(on_event::<TickEvent>()),
                (
                    action::apply_completed_action_markers,
                    apply_deferred,
                    action::on_success::apply_move,
                    action::on_success::apply_attack,
                    // ...
                    action::set_state_await_anim,
                    apply_deferred, // .in_set(CustomFlush),
                )
                    .chain()
                    .in_set(ActionSet::Apply)
                    .run_if(on_event::<ActionCompleteEvent>()),
            ),
        )
        .add_systems(
            Update,
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
                .in_set(ActionSet::Animate)
                .run_if(in_state(ActionSystemState::AwaitAnim)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
