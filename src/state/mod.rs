use bevy::prelude::*;

// State
// it'd be nice if we could ensure only valid state transitions ..
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    InitAssets,
    InitBoard,
    InitPlayer,
    InitStage,
    LoadAssets,
    InitUI,
    InitTileMap,
    InitMobs,
    Game,
}
