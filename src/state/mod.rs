use bevy::prelude::*;

// use crate::graphics::player_avatar::PlayerAvatarRes;

// State
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    SpawnPlayerAvatar,
    Ready,
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

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, States)]
#[allow(dead_code)]
pub(crate) enum TickState {
    #[default]
    PlayerInput,
    ValidatePlayerAction,
    //
    PrepareAgentActions,
    //
    ClockTick, // advance clock
    //
    PlayerActionTick,
    //
    AgentActionsTick,
    //
    ApplyCompletedActions,
    //
    Animate,
}
