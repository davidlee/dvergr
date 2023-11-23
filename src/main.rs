pub mod action;
pub mod anatomy;
pub mod attributes;
pub mod board;
pub mod damage;
pub mod dice;
pub mod ui;
pub mod sys {
    pub mod player_movement;
}
pub mod map;
pub mod player;
pub mod state;
pub mod time;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution, WindowTheme};
use bevy_pancam::PanCamPlugin;
use bevy_turborand::prelude::RngPlugin;
use board::*;
use map::*;
use state::AppState;
use sys::player_movement::*;
use time::TimePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "One day I will be a roguelike".into(),
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
        .add_plugins(PanCamPlugin::default())
        .add_plugins(TimePlugin)
        .add_plugins(BoardPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(RngPlugin::default())
        .add_state::<AppState>()
        .add_systems(Startup, ui::spawn_camera)
        .add_systems(OnEnter(AppState::DrawUI), ui::spawn_layout)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, keybindings)
        .add_event::<PlayerMovementEvent>()
        .run();
}
