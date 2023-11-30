use crate::state::AppInitEvent;
use crate::AppState;
use bevy::prelude::*;
use bevy_pancam::PanCam;

/*
pallette:

006466 065A60 0B525B 144552 1B3A4B
212F45 272640 312244 3E1F47 4D194D
*/

#[derive(Component, Debug)]
pub struct MapViewPanel;

#[derive(Component, Debug)]
pub struct MapViewContainer;

pub fn spawn_layout_shim(mut ev_writer: EventWriter<AppInitEvent>) {
    ev_writer.send(AppInitEvent::SetAppState(AppState::InitTileMap));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        min_scale: 0.1,
        max_scale: Some(2.),
        ..default()
    });
}
