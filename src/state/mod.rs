use bevy::prelude::*;

// State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
#[allow(dead_code)]
pub enum AppState {
    #[default]
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
