use crate::typical::*;
use bevy::prelude::Color;
// Cell
//
// a cell is taller than it is wide / deep; about the size a man can stand in.
#[allow(dead_code)]
const CELL_DIMENSIONS_METRES: [f32; 3] = [0.5, 0.5, 2.0];

#[derive(PartialEq, Clone, Debug, Component)]
pub struct Cell {
    pub material: CellMaterial,
    pub floor: CellFloor,
    pub feature: Option<Entity>, // door, trap, statue, well, etc
    pub items: CellItems,
    pub material_blocks_visibility: bool,
    pub blocks_visibility_computed: bool,
    pub light_intensity: f64,
    pub light_color: Color,
    pub position: IVec3,
}

impl Cell {}

// type aliases
pub type CellMaterial = Option<Material>;
pub type CellFloor = Option<Material>;
pub type CellItems = Option<Vec<Entity>>;

impl Cell {
    pub fn empty(position: IVec3) -> Self {
        Cell {
            material_blocks_visibility: false,
            blocks_visibility_computed: false,
            position,
            ..default()
        }
    }

    pub fn wall(xyz: IVec3) -> Self {
        Cell {
            material: Some(Material::Dirt),
            material_blocks_visibility: true,
            blocks_visibility_computed: true,
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
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            material: None,
            floor: None,
            feature: None,
            items: Some(vec![]),
            material_blocks_visibility: false,
            blocks_visibility_computed: false,
            light_intensity: 0.0,
            light_color: Color::NONE,
            position: IVec3::new(-1, -1, -1),
        }
    }
}
