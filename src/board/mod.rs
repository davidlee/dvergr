pub mod cell;
pub mod cell_store;
pub mod creature_store;
pub mod direction;
pub mod geometry;
pub mod plugin;
pub mod primitives;

// pub use plugin::BoardPlugin;
// pub use direction::Direction;
// pub use primitives::*;
// pub use geometry::*;
pub use cell_store::*;
pub use creature_store::*;
// pub use cell::*;

pub const BOARD_SIZE_X: i32 = 48;
pub const BOARD_SIZE_Y: i32 = 24;
pub const BOARD_SIZE_Z: i32 = 1;

use crate::typical::*;
// Board
//
#[derive(Clone, Debug, Resource)]
#[allow(dead_code)]
pub struct Board {
    pub size: Size3d,
    pub cell_store: CellStore,
    pub creature_store: CreatureStore,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            size: Size3d {
                width: BOARD_SIZE_X,
                height: BOARD_SIZE_Y,
                depth: 1,
            },
            cell_store: CellStore::default(),
            creature_store: CreatureStore::default(),
        }
    }
}
impl Board {
    pub fn coords(&self) -> Vec<IVec3> {
        let mut cv = vec![];
        for z in 0..BOARD_SIZE_Z {
            for y in 0..BOARD_SIZE_Y {
                for x in 0..BOARD_SIZE_X {
                    cv.push(IVec3::new(x, y, z));
                }
            }
        }
        cv
    }

    pub fn apply_direction(&self, pos: &IVec3, direction: &Direction) -> Result<IVec3, &str> {
        let [x, y, z] = pos.to_array();
        let [dx, dy, dz] = direction.offset().to_array();
        let [x, y, z] = [x + dx, y + dy, z + dz];

        if [x, y, z].iter().any(|n| *n < 0)
            || x > self.size.width
            || y > self.size.height
            || z > self.size.depth
        {
            return Err("out of bounds");
        } else {
            Ok(IVec3::new(x, y, z))
        }
    }
}

// Position
//
#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum Position {
    Area(Area3d),
    Point(IVec3),
}

// PlayerCellVisibility
//
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct PlayerCellVisibility {
    pub seen: bool,
    pub visible: bool,
    pub position: IVec3,
}

impl PlayerCellVisibility {
    pub fn new(position: IVec3) -> Self {
        Self {
            visible: false,
            seen: false,
            position,
        }
    }
}

// Material
//
#[derive(Default, Resource, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum Material {
    #[default]
    Dirt,
    Sandstone,
    Granite,
    Marble,
    Quartz,
    Sand,
}
