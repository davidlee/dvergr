use crate::typical::*;

use crate::creature::Stance;
// Locus
//

#[derive(Component, Debug, Clone, PartialEq)]
pub(crate) struct Locus {
    pub(crate) position: Position,
    pub(crate) velocity: Vec3,
    pub(crate) direction: Direction,
    pub(crate) facing: Direction,
    pub(crate) stance: Stance,
    pub(crate) pace: Pace,
    pub(crate) weight: f64,
}

impl Locus {
    pub(crate) fn set_pos(&mut self, pos: IVec3) {
        self.position = Position::Point(pos);
    }

    pub(crate) fn set_area(&mut self, area: Area3d) {
        self.position = Position::Area(area);
    }
}

impl Default for Locus {
    fn default() -> Self {
        Locus {
            position: Position::Point(IVec3::new(0, 0, 0)),
            velocity: Vec3::new(0., 0., 0.),
            direction: Direction::North,
            facing: Direction::North,
            stance: Stance::Standing,
            pace: Pace::default(),
            weight: 80.0,
        }
    }
}
