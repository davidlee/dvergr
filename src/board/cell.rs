use crate::typical::*;
// use bevy::prelude::Color;
// Cell
//
// a cell is taller than it is wide / deep; about the size a man can stand in.
#[allow(dead_code)]
const CELL_DIMENSIONS_METRES: [f32; 3] = [0.5, 0.5, 2.0];

#[derive(PartialEq, Clone, Debug, Component)]
pub struct Cell {
    pub material: Option<Entity>,
    pub floor: Option<Entity>,
    pub feature: Option<Entity>, // door, trap, statue, well, etc
    pub items: Option<Entity>,

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

#[derive(Bundle, Debug, Copy, Clone, Eq, PartialEq)]
struct CellBundle {
    pub cell: Cell,
    pub cell_material: CellFillMaterial,
    // pub items:
}

impl Cell {}

// type aliases
pub type CellMaterial = Option<Material>;
pub type CellFloor = Option<Material>;
pub type CellItems = Option<Vec<Entity>>;

impl Cell {
    pub fn empty(position: IVec3) -> Self {
        Cell {
            // material_blocks_visibility: false,
            // blocks_visibility_computed: false,
            position,
            ..default()
        }
    }

    pub fn wall(xyz: IVec3) -> Self {
        Cell {
            // material: Some(Material::Dirt),
            // material_blocks_visibility: true,
            // blocks_visibility_computed: true,
            position: xyz,
            ..default()
        }
    }

    pub fn passable(&self) -> bool {
        self.material.is_none()
    }

    pub fn impassable(&self) -> bool {
        !self.passable()
    }

    pub fn blocks_visibility(&self) -> bool {
        self.material.is_some() // and is not ... glass?
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            material: None,
            floor: None,
            feature: None,
            items: None,
            // items: Some(vec![]),
            // material_blocks_visibility: false,
            // blocks_visibility_computed: false,
            // light_intensity: 0.0,
            // light_color: Color::NONE,
            position: IVec3::new(-1, -1, -1),
        }
    }
}

// Material
//
#[derive(Default, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum Material {
    #[default]
    Dirt,
    Sandstone,
    Granite,
    Marble,
    Quartz,
    Sand,
}
