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
    SetAppStateDeferred(AppState, usize),
}

pub fn handle_app_init_event(
    mut reader: EventReader<AppInitEvent>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ev in reader.read() {
        match ev {
            AppInitEvent::SetAppState(ns) => {
                let current_state = state.get();
                warn!("State Transition: {:?} --> {:?}", current_state, ns);
                next_state.set(*ns);
            }
            AppInitEvent::SetAppStateDeferred(_ns, _delay) => (),
        }
    }
}

// pub fn set_state_spawn_player_avatar(
//     mut ev_writer: EventWriter<AppInitEvent>,
//     avatar_ref: Res<PlayerAvatarRes>,
// ) {
//     // dbg(avatar_ref.entity);
//     ev_writer.send(AppInitEvent::SetAppState(AppState::SpawnPlayerAvatar));
// }
