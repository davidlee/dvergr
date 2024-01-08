use crate::typical::*;

pub mod pace;
// pub mod skills;

pub use pace::*;

#[derive(Bundle, Debug, Clone)]
pub struct CharacterBundle {
    pub character: Character,
    pub pace: Pace,
}

#[derive(Component, Debug, Clone)]
pub struct Character;
