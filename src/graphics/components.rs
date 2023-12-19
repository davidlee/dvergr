use crate::typical::*;
// use bevy::prelude::SpatialBundle;

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelSize {
    pub width: f32,
    pub height: f32,
}
pub type TileSize = PixelSize;

#[derive(Component, Debug, Copy, Clone)]
pub struct GridSize {
    pub width: i32,
    pub height: i32,
}

// #[derive(Component, Debug, Default)]
// pub struct Stage;

// #[derive(Bundle, Debug, Default)]
// pub struct StageBundle {
//     stage: Stage,
// }

// pub fn spawn_stage(mut commands: Commands, mut ev_writer: EventWriter<AppInitEvent>) {
//     debug!("[AppState::InitStage] spawn_stage");
//     commands.spawn((StageBundle::default(), SpatialBundle::default()));

//     ev_writer.send(AppInitEvent::SetAppState(AppState::LoadAssets));
// }
