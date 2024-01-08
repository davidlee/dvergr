use crate::typical::*;

pub struct Meta {
    entity: Entity,
    target: Option<Entity>,
}

trait CommandDictionary {}

enum CreatureCommand {
    WaitSeconds {
        entity: Entity,
        seconds: f32,
    }, // seconds
    Move {
        entity: Entity,
        direction: Direction,
        pace: Pace,
    },
    Attack {
        // ?
        entity: Entity,
        direction: Direction,
        target: Entity,
        maneuver: (),
        weapon: (),
    },
    Drink {
        entity: Entity,
        target: Entity,
    },
}

impl CommandDictionary for CreatureCommand {}

enum PlayerCommand {}
impl CommandDictionary for PlayerCommand {}

enum Command<T: Component> {
    Valid {
        command: CreatureCommand<T>,
        seconds: f32,
    }, // seconds
    Invalid {
        command: CreatureCommand<T>,
        message: String,
    },
}
