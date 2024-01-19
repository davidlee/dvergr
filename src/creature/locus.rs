use crate::typical::*;

use crate::creature::Stance;
// Locus
//

#[derive(Component, Debug, Clone, PartialEq)]
pub(crate) struct Locus {
    pub(crate) position: IVec3,
    pub(crate) velocity: Vec3,
    pub(crate) direction: Direction,
    pub(crate) facing: Direction,
    pub(crate) stance: Stance,
    pub(crate) pace: Pace,
    pub(crate) weight: f64,
}

impl Default for Locus {
    fn default() -> Self {
        Locus {
            position: IVec3::ZERO,
            velocity: Vec3::ZERO,
            direction: Direction::North,
            facing: Direction::North,
            stance: Stance::Standing,
            pace: Pace::default(),
            weight: 80.0,
        }
    }
}
