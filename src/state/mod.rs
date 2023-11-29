use bevy::prelude::*;

// State
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
                info!("State Transition: {:?} --> {:?}", current_state, ns);
                next_state.set(*ns);
            }
            AppInitEvent::SetAppStateDeferred(_ns, _delay) => (),
        }
    }
}
