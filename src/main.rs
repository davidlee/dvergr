// #![allow(dead_code)]

pub(crate) mod action;
pub(crate) mod board;
pub(crate) mod combat;
pub(crate) mod creature;
pub(crate) mod dice;
pub(crate) mod goblin;
pub(crate) mod graphics;
pub(crate) mod input;
pub(crate) mod inventory;
pub(crate) mod marker_components;
pub(crate) mod material;
pub(crate) mod player;
pub(crate) mod time;
pub(crate) mod typical;

use bevy::window::{PresentMode, WindowResolution, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_turborand::prelude::RngPlugin;
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

// #[derive(States, Debug, Default, Hash, Eq, Clone, PartialEq)]
// enum AssetLoadState {
//     #[default]
//     Loading,
//     Ready,
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
struct CustomFlush;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ActorBehaviour;

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
        .add_plugins(DefaultPickingPlugins)
        // RESOURCES
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Msaa>()
        .init_resource::<Board>()
        // STATE
        .add_state::<ActionSystemState>()
        .add_state::<PlayerInputState>()
        // .add_state::<AssetLoadState>()
        // EVENTS
        .add_event::<SpawnPlayerEvent>()
        .add_event::<TickEvent>()
        .add_event::<PlayerInputRequestEvent>()
        .add_event::<ActionInvalidatedEvent>()
        .add_event::<ActionValidatedEvent>()
        .add_event::<ActionPlanRequestEvent>()
        .add_event::<ActionCompleteEvent>()
        .add_event::<ActionAddedEvent>()
        .add_event::<ActionStartedEvent>()
        .add_event::<ActionAbortedEvent>()
        .add_event::<StillWaitForAnimEvent>()
        .add_event::<SpawnGoblinEvent>()
        //
        // SYSTEMS
        //
        // Startup
        //
        .add_systems(
            Startup,
            (
                graphics::load_spritesheets,
                board::generator::populate_board,
                board::generator::add_fun,
                apply_deferred,
                graphics::spawn_voxel_map,
                apply_deferred,
                player::spawn_player_and_3d_elements,
                goblin::spawn_goblins,
                apply_deferred,
                graphics::spawn_player_sprite_and_2d_camera,
                action::bootstrap,
            )
                .chain(),
        )
        //
        // Actions
        //
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
        .add_systems(
            OnEnter(ActionSystemState::Plan),
            action::plan_init_check_or_tick,
        )
        .add_systems(
            PreUpdate,
            (
                (
                    action::plan_init_check_or_tick.run_if(
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
                    apply_deferred,
                    action::plan_init_check_or_tick, // proceed to next tick?
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
                    apply_deferred,
                )
                    .chain()
                    .in_set(ActionSet::Apply)
                    .run_if(on_event::<ActionCompleteEvent>()),
            ),
        )
        .add_systems(
            Update,
            (
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
        .add_systems(
            Update,
            (
                graphics::torchlight::flicker_torches,
                graphics::move_anim::animate_player_fov,
            ),
        )
        .add_systems(
            PostUpdate,
            goblin::spawn_goblins.run_if(on_event::<SpawnGoblinEvent>()),
        )
        .add_systems(
            Update,
            instrument_time.run_if(in_state(PlayerInputState::Inactive)),
        )
        .run();
}

fn instrument_time(
    frame: Res<FrameCount>,
    tick: Res<TickCount>,
    res: Res<State<ActionSystemState>>,
) {
    if frame.0 % 10 == 0 {
        warn!(
            "😈 frame / tick / state: {:?} / {:?} => {:?}",
            frame.0,
            tick.0,
            res.get()
        );
    }
}
