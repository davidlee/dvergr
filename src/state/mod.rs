use bevy::prelude::*;

// State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    InitAssets,
    LoadAssets,
    DrawUI,
    Game,
    // PrepareBoard,
    // SetupLogicalMap,
    // LoadTextures,
    // CreateCharacter,
    // Prepare,
    // BuildWorld,
    // PopulateMap,
    // Embark,
    // GameOver,
}
