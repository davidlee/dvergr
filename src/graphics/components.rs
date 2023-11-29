use crate::typical::*;
use bevy::prelude::SpatialBundle;

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

#[derive(Component, Debug, Copy, Clone)]
pub struct PixelPos {
    pub x: f32,
    pub y: f32,
}
#[derive(Component, Debug, Default)]
pub struct Stage;

#[derive(Bundle, Debug, Default)]
pub struct StageBundle {
    stage: Stage,
}

pub fn spawn_stage(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,

    state: Res<State<AppState>>,
) {
    println!("[AppState::InitStage] spawn_stage");
    commands.spawn((StageBundle::default(), SpatialBundle::default()));

    match state.get() {
        AppState::InitStage => next_state.set(AppState::LoadAssets),
        s => panic!("illegal state: {:?}", s),
    }
}
