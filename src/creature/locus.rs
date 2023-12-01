use crate::typical::*;

use crate::creature::Stance;
// Locus
//

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Locus {
    pub position: Position,
    pub velocity: Vec3,
    pub direction: Direction,
    pub facing: Direction,
    pub stance: Stance,
    pub weight: f64,
}

impl Locus {
    pub fn set_pos(&mut self, pos: IVec3) {
        self.position = Position::Point(pos);
    }

    pub fn set_area(&mut self, area: Area3d) {
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
            weight: 80.0,
        }
    }
}
