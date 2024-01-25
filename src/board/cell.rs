use crate::material::*;
use crate::typical::*;

// Cell
//
// a cell is taller than it is wide / deep; about the size a man can stand in.
#[allow(dead_code)]
const CELL_DIMENSIONS_METRES: [f32; 3] = [0.5, 0.5, 2.0];

#[derive(PartialEq, Clone, Debug, Component)]
pub(crate) struct Cell {
    pub(crate) position: IVec3,
}

#[derive(Bundle, Debug, Clone, PartialEq)]
pub(crate) struct CellWallBundle {
    pub(crate) cell: Cell,
    pub(crate) cell_material: Wall,
    pub(crate) cell_items: CellItems,
}

// TODO allow empty cells with neighbouring wall cells to have items / features on the wall itself
// maybe with a HashMap of Direction => Entity
//

// pub(crate) struct CellFloorBundle {
//     pub(crate) cell: Cell,
//     pub(crate) cell_floor: Floor,
//     pub(crate) cell_items: CellItems,
//     pub(crate) cell_wall_features: (),
//     pub(crate) cell_wall_items: (),
// }

// TODO implement gravity
// a cell without floor or material underneath can only contain items temporarily; they'll fall through
// pub(crate) struct CellEmptyBundle {
//     pub(crate) cell: Cell,
//     pub(crate) cell_items: CellItems,
// }

impl Cell {
    pub(crate) fn new(x: i32, y: i32, z: i32) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub(crate) struct Wall {
    pub(crate) substance: Substance,
    pub(crate) position: IVec3,
}

impl Wall {
    // pub(crate) fn blocks_visibility(&self) -> bool {
    //     true
    // }

    // pub(crate) fn impassable(&self) -> bool {
    //     true
    // }

    pub(crate) fn new(x: i32, y: i32, z: i32, material: Substance) -> Self {
        let position = IVec3::new(x, y, z);
        Self {
            position,
            substance: material,
        }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) struct CellFeature {
    entity: Entity,
    position: IVec3,
}

// impl CellFeature {
//     pub(crate) fn blocks_visibility(&self) -> bool {
//         false
//     }

//     pub(crate) fn impassable(&self) -> bool {
//         false
//     }
//     pub(crate) fn new(x: i32, y: i32, z: i32, entity: Entity) -> Self {
//         let position = IVec3::new(x, y, z);
//         Self { position, entity }
//     }
// }

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub(crate) struct CellItems {
    items: Vec<Entity>,
    position: IVec3,
}

// impl CellItems {
//     pub(crate) fn new(x: i32, y: i32, z: i32) -> Self {
//         let position = IVec3::new(x, y, z);
//         let items = vec![];
//         Self { position, items }
//     }
// }

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub(crate) struct Floor {
    material: Substance,
    position: IVec3,
}

impl Floor {
    pub(crate) fn new(x: i32, y: i32, z: i32, material: Substance) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position, material }
    }
}
