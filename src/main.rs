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

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution, WindowTheme};
use bevy_pancam::PanCamPlugin;
use bevy_turborand::prelude::RngPlugin;
use state::AppState;

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
        .add_plugins(PanCamPlugin::default())
        .add_plugins(time::TimePlugin)
        .add_plugins(RngPlugin::default())
        .add_plugins(board::BoardPlugin)
        .add_plugins(graphics::StagePlugin)
        .add_plugins(graphics::AssetLoadingPlugin)
        .add_plugins(graphics::TileMapPlugin)
        .add_plugins(graphics::MobsPlugin)
        .add_plugins(player::PlayerPlugin)
        // systems
        .add_systems(Startup, ui::spawn_camera)
        .add_systems(OnEnter(AppState::InitUI), ui::spawn_layout)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, input::keybindings)
        // events
        .run();
}
