use crate::anatomy::*;
use crate::typical::*;

pub mod pace;
// pub mod skills;

pub use pace::*;

#[derive(Bundle, Debug, Clone)]
pub struct CharacterBundle {
    pub character: Character,
    pub pace: Pace,
    pub gender: Gender,
    pub age: Age,
    pub species: Species,
}

#[derive(Component, Debug, Clone)]
pub struct Character {
    name: Option<String>,
    skills: (),
    abilities: (),
    // proficiencies: (),
    spells: (),
    level: CharacterLevel,
}

#[derive(Component, Debug, Clone)]
pub struct Age(pub u16);

#[derive(Component, Debug, Clone)]
pub struct CharacterLevel(pub u16);
