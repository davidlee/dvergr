use crate::typical::*;
// use bevy::prelude::Color;
// Cell
//
// a cell is taller than it is wide / deep; about the size a man can stand in.
#[allow(dead_code)]
const CELL_DIMENSIONS_METRES: [f32; 3] = [0.5, 0.5, 2.0];

#[derive(PartialEq, Clone, Debug, Component)]
pub struct Cell {
    // pub material: Option<Entity
    // pub floor: Option<Entity>,
    // pub feature: Option<Entity>, // door, trap, statue, well, etc
    // pub items: Option<Entity>,

    // pub material_blocks_visibility: bool,
    // pub blocks_visibility_computed: bool,
    // pub light_intensity: f64,
    // pub light_color: Color,
    pub position: IVec3,
}

/*
TODO experiment with a Cell which references other entities for:
    - cell wall / material
    - cell floor
    - items
    - light source
    - cell feature
    - illumination


we'd want a CellBundle ...

*/

// #[derive(Component)]
// type MaybeCellFillMaterial = Option<CellFillMaterial>;

#[derive(Bundle, Debug, Clone, PartialEq)]
pub struct CellWallBundle {
    pub cell: Cell,
    pub player_visibility: PlayerCellVisibility,
    pub cell_material: Wall,
    // can have floor
    pub cell_items: CellItems,
}

pub struct CellFloorBundle {
    pub cell: Cell,
    pub player_visibility: PlayerCellVisibility,
    pub cell_floor: Floor,
    pub cell_items: CellItems,
    // can have feature
}

// TODO cell with no material / floor cannot have items; they fall through

impl Cell {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let position = IVec3::new(x, y, z);
        Self { position }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            position: IVec3::new(-1, -1, -1),
        }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Wall {
    material: Material,
    position: IVec3,
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

// Material
//
#[derive(Component, Default, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum Material {
    #[default]
    Dirt,
    Sandstone,
    Granite,
    Marble,
    Quartz,
    Sand,
}
