use crate::material::*;
use crate::typical::*;
// use bevy::prelude::Color;
// Cell
//
// a cell is taller than it is wide / deep; about the size a man can stand in.
#[allow(dead_code)]
const CELL_DIMENSIONS_METRES: [f32; 3] = [0.5, 0.5, 2.0];

#[derive(PartialEq, Clone, Debug, Component)]
pub struct Cell {
    pub position: IVec3,
}

#[derive(Bundle, Debug, Clone, PartialEq)]
pub struct CellWallBundle {
    pub cell: Cell,
    pub cell_material: Wall,
    pub cell_items: CellItems,
}

// TODO allow empty cells with neighbouring wall cells to have items / features on the wall itself
// maybe with a HashMap of Direction => Entity
//
pub struct CellFloorBundle {
    pub cell: Cell,
    pub cell_floor: Floor,
    pub cell_items: CellItems,
    pub cell_wall_features: (),
    pub cell_wall_items: (),
}

// TODO implement gravity
// a cell without floor or material underneath can only contain items temporarily; they'll fall through
pub struct CellEmptyBundle {
    pub cell: Cell,
    pub cell_items: CellItems,
}

impl Cell {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Wall {
    pub material: Material,
    pub position: IVec3,
}

impl Wall {
    pub fn blocks_visibility(&self) -> bool {
        true
    }

    pub fn impassable(&self) -> bool {
        true
    }

    pub fn new(x: i32, y: i32, z: i32, material: Material) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position, material }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Copy, Clone)]
pub struct CellFeature {
    entity: Entity,
    position: IVec3,
}

impl CellFeature {
    pub fn blocks_visibility(&self) -> bool {
        false
    }

    pub fn impassable(&self) -> bool {
        false
    }
    pub fn new(x: i32, y: i32, z: i32, entity: Entity) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position, entity }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct CellItems {
    items: Vec<Entity>,
    position: IVec3,
}

impl CellItems {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let position = IVec3::new(x, y, z);
        let items = vec![];
        Self { position, items }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Floor {
    material: Material,
    position: IVec3,
}

impl Floor {
    pub fn new(x: i32, y: i32, z: i32, material: Material) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position, material }
    }
}
