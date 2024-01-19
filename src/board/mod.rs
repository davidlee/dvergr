pub mod cell;
pub mod cell_store;
pub mod creature_store;
pub mod direction;
pub mod generator;
pub mod geometry;
pub mod lighting;
pub mod primitives;

pub use cell_store::*;
pub use creature_store::*;

pub const BOARD_SIZE_X: i32 = 48;
pub const BOARD_SIZE_Y: i32 = 24;
pub const BOARD_SIZE_Z: i32 = 1;

use crate::typical::*;

type Size3d = IVec3;

// Board
//
#[derive(Clone, Debug, Resource)]
#[allow(dead_code)]
pub(crate) struct Board {
    pub(crate) size: Size3d,
    pub(crate) cell_store: EntityPositionStore,
    pub(crate) wall_store: EntityPositionStore,
    pub(crate) floor_store: EntityPositionStore,
    pub(crate) feature_store: EntityPositionStore,
    pub(crate) visibility_store: EntityPositionStore,
    pub(crate) items_store: EntityPositionStore,
    pub(crate) creature_store: CreatureStore,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            size: Size3d {
                x: BOARD_SIZE_X,
                y: BOARD_SIZE_Y,
                z: 1,
            },
            cell_store: EntityPositionStore::default(),
            wall_store: EntityPositionStore::default(),
            floor_store: EntityPositionStore::default(),
            feature_store: EntityPositionStore::default(),
            items_store: EntityPositionStore::default(),
            visibility_store: EntityPositionStore::default(),
            creature_store: CreatureStore::default(),
        }
    }
}

impl Board {
    pub(crate) fn coords(&self) -> Vec<IVec3> {
        let mut cv = vec![];
        for z in 0..self.size.z {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    cv.push(IVec3::new(x, y, z));
                }
            }
        }
        cv
    }

    pub(crate) fn apply_direction(
        &self,
        pos: &IVec3,
        direction: &Direction,
    ) -> Result<IVec3, &str> {
        let [x, y, z] = pos.to_array();
        let [dx, dy, dz] = direction.offset().to_array();
        let [x, y, z] = [x + dx, y + dy, z + dz];

        if [x, y, z].iter().any(|n| *n < 0) || x > self.size.x || y > self.size.y || z > self.size.z
        {
            Err("out of bounds")
        } else {
            Ok(IVec3::new(x, y, z))
        }
    }

    // FIXME check for things other than walls - statues, pillars, creatures, doors ...

    pub(crate) fn is_unoccupied(&self, pos: &IVec3) -> bool {
        self.wall_store.get(&pos).is_none()
    }
}
