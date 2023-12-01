use crate::typical::*;

pub mod equipment;
pub mod pace;
// pub mod skills;

pub use equipment::Equipment;
pub use pace::*;

#[derive(Bundle, Debug, Clone)]
pub struct CharacterBundle {
    pub character: Character,
    pub equipment: Equipment,
    pub pace: Pace,
    // pub skills: (),
}

#[derive(Component, Debug, Clone)]
pub struct Character;
