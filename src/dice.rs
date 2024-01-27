use bevy_turborand::prelude::*;

pub const D10_VALUES: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

#[derive(Clone)]
pub struct Dice {
    rng: Rng,
}

impl Default for Dice {
    fn default() -> Self {
        Self { rng: Rng::new() }
    }
}

impl Dice {
    pub fn d10(&self) -> &u8 {
        self.rng.sample(&D10_VALUES).unwrap()
    }
}
