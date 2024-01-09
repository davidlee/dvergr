use crate::typical::*;

pub struct Meta {
    entity: Entity,
    issuer: Issuer,
    target: Option<Entity>,
}

enum Issuer {
    Player,
    Character,
    Creature,
}

#[derive(Event, Debug, Clone)]
enum Command {
    WaitSeconds {
        meta: Meta,
        seconds: f64,
    }, // seconds
    Move {
        entity: Entity,
        direction: Direction,
        pace: Pace,
    },
    Attack {
        // ?
        meta: Meta,
        direction: Direction,
        target: Entity,
        // maneuver: (),
        // weapon: (),
    },
    Drink {
        meta: Meta,
        target: Entity,
    },
}

enum ValidatedCommand {
    Valid {
        command: CreatureCommand,
        seconds: f64,
    }, // seconds
    Invalid {
        command: CreatureCommand,
        message: String,
    },
}
