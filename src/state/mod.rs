use bevy::prelude::*;

// use crate::graphics::player_avatar::PlayerAvatarRes;

// State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    SpawnPlayerAvatar,
    Ready,
    // InitBoard,
    // InitStage,
    // LoadAssets,
    // InitUI,
    // InitTileMap,
    // InitMobs,
}

#[derive(Debug, Event)]
pub enum AppInitEvent {
    SetAppState(AppState),
}

pub fn handle_app_init_event(
    mut reader: EventReader<AppInitEvent>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ev in reader.read() {
        let current_state = state.get();
        let AppInitEvent::SetAppState(new_state) = ev;
        warn!("State Transition: {:?} --> {:?}", current_state, new_state);
        next_state.set(*new_state);
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    Update,
    // Animate,
}
