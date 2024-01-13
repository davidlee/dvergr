use bevy::prelude::Event;

#[derive(Event, Debug, Eq, PartialEq, PartialOrd, Clone)]
pub enum Verb {
    Face,
    // Crawl,
    Walk,
    Run,
    Sprint,
    Climb,
    Jump,
    // Swim,
    // Ride,
    //
    Take,
    Stow,
    Wear,
    Remove,
    Pour,
    Fill,
    Give,
    Put, // in / on
    Burn,
    Light,

    Look,
    Examine,
    Search,
    Smell,

    Eat,
    Drink,
    Inhale,

    Ask,
    Talk,
    Sing,
    Pray,
    Curse,
    Listen,

    // Sell,
    // Buy,
    Open,
    Close,
    Lock,
    Unlock,
    Knock,
    Move,
    Pull,
    Push,
    Turn,
    Break,
    Feel,
    Touch,
    Tie,
    Untie,

    Again,
    Wait,
}
