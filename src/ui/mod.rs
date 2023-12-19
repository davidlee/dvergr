use crate::state::AppInitEvent;
use crate::AppState;
use bevy::prelude::*;

pub mod layout;
pub use layout::spawn_layout;

pub fn spawn_layout_shim(mut ev_writer: EventWriter<AppInitEvent>) {
    ev_writer.send(AppInitEvent::SetAppState(AppState::InitTileMap));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
