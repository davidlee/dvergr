use crate::creature::pace::*;
use crate::time::*;
use bevy::prelude::*;
// Actions
//
#[derive(Component, Debug, Clone, Default)]
pub struct ActionList {
    pub time: TurnTime, // local time is compared to TurnTime global / resource to check when can act
    pub current_action: Option<Command>,
    pub queued_commands: Vec<Command>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Command {
    Combat(CombatAction),
    Move(MoveAction),
    Item(ItemAction),
    Wait(Duration),
    // AwaitCue(),
    Talk(Entity),
    Look(Entity),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MoveAction {
    Slither(Direction),
    Crawl(Direction),
    Sneak(Direction),
    Probe(Direction), // find traps
    SneakilyProbe(Direction),
    // usually
    Walk(Direction),
    Jog(Direction),
    Run(Direction),
    Sprint(Direction),
    Barge(Direction), // breach door
    Climb(Direction),
    Prone(),
    Kneel(),
    Crouch(),
    Stand(),
    Swim(),
    Jump(),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ItemAction {
    Drop(Entity),
    Retrieve(Entity),
    Get(Entity),
    Wear(Entity),
    // Wield(Entity),
    Draw(Entity),
    Equip(Entity),
    Use(Entity),
    Drink(Entity),
    Eat(Entity),
    Quaff(Entity),
    Throw(Entity),
    Inspect(Entity),
    Identify(Entity),
    Look(Entity),
    Craft(),
    Cook(),
    Pour(),
    Fill(),
    Stow(),
    Wash(),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum CombatAction {
    Attack {
        actor: Entity,
        target: Entity,          // creature or cell (area of effect)
        weapon: Entity,          // weapon
        kind: String,            // verb
        variant: Option<String>, // manoeuvre
        moving: Option<Direction>,
        tempo: Option<Tempo>,
    },
    MissileAttack {
        actor: Entity,
        target: Entity,
        weapon: Entity,          // weapon
        ammo: Option<Entity>,    // ammo
        kind: String,            // verb
        variant: Option<String>, // manoeuvre
        moving: Option<Direction>,
        tempo: Option<Tempo>,
    },
    Defend {
        actor: Entity,
        kind: String,            // verb
        variant: Option<String>, // manoeuvre
        item: Option<Entity>,    // weapon
        tempo: Option<Tempo>,
    },
    Await {
        actor: Entity,
        target: Option<Entity>,
        cue: String,             // verb
        variant: Option<String>, // manoeuvre
    },
}
