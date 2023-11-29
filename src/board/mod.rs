use bevy::math::IVec3;
use bevy::prelude::{default, Component, Entity, Resource};
use bevy::render::color::Color;
use std::collections::HashMap;

pub mod direction;
pub use direction::Direction;

pub mod plugin;
pub use plugin::BoardPlugin;

pub mod primitives;
pub use primitives::*;

pub mod geometry;
pub use geometry::*;

pub const BOARD_SIZE_X: i32 = 48;
pub const BOARD_SIZE_Y: i32 = 24;
pub const BOARD_SIZE_Z: i32 = 1;

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

// CellStore
//
#[derive(Resource, Eq, PartialEq, Clone, Debug, Default)]
pub struct CellStore {
    // TODO make this IVec3, Entity
    to_entity: HashMap<IVec3, Entity>,
    to_uvec: HashMap<Entity, IVec3>,
}

impl CellStore {
    pub fn set(&mut self, pos: IVec3, entity: Entity) {
        self.to_entity.insert(pos, entity);
        self.to_uvec.insert(entity, pos);
    }

    pub fn get(&self, pos: &IVec3) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn get_pos(&self, entity: &Entity) -> Option<&IVec3> {
        self.to_uvec.get(entity)
    }

    pub fn remove(&mut self, pos: &IVec3) -> Option<Entity> {
        if let Some(entity) = self.to_entity.remove(pos) {
            self.to_uvec.remove(&entity);
            Some(entity)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IVec3, &Entity)> {
        self.to_entity.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&IVec3, &mut Entity)> {
        self.to_entity.iter_mut()
    }
}

// Position
//
#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum Position {
    Area(Area3d),
    Point(IVec3),
}

// CreatureStore
//

#[derive(Resource, Clone, Debug)]
#[allow(dead_code)]
#[derive(Default)]
pub struct CreatureStore {
    to_entity: HashMap<IVec3, Entity>,
    // this is the source of truth for #to_entity
    to_area: HashMap<Entity, Vec<IVec3>>,
}

impl CreatureStore {
    pub fn add(&mut self, entity: Entity, area: Area3d) -> Result<(), &str> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.to_area.entry(entity) {
            e.insert(area.clone());

            for pos in area.clone() {
                self.to_entity.insert(pos, entity);
            }

            Ok(())
        } else {
            Err("already exists")
        }
    }

    pub fn add_single(&mut self, entity: Entity, pos: IVec3) -> Result<(), &str> {
        self.add(entity, vec![pos])
    }

    pub fn update(&mut self, entity: Entity, area: Area3d) -> Result<(), &str> {
        if !self.to_area.contains_key(&entity) {
            Err("expected to already exist, but is missing")
        } else {
            let prev_area = self.to_area.get(&entity).unwrap();

            for p in prev_area.iter().filter(|p| !area.contains(p)) {
                self.to_entity.remove(p);
            }

            for p in area.iter().filter(|p| !prev_area.contains(p)) {
                self.to_entity.insert(*p, entity);
            }

            self.to_area.insert(entity, area);

            Ok(())
        }
    }

    pub fn update_single(&mut self, entity: Entity, pos: IVec3) -> Result<(), &str> {
        self.update(entity, vec![pos])
    }

    pub fn get_entity_at(&self, pos: &IVec3) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn get_area_for(&self, entity: &Entity) -> Option<&Area3d> {
        self.to_area.get(entity)
    }

    pub fn get_pos_for(&self, entity: &Entity) -> Option<&IVec3> {
        match self.to_area.get(entity) {
            Some(vec) => {
                if vec.len() == 1 {
                    Some(&vec[0])
                } else {
                    panic!("Area was not single pos");
                }
            }
            None => None,
        }
    }
}

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

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct PlayerCellVisibility {
    pub seen: bool,
    pub visible: bool,
    pub position: IVec3,
}

// impl Default for PlayerCellVisibility {
//     fn default() -> Self {
//         Self {
//             seen: false,
//             visible: false,

//         }
//     }
// }

impl PlayerCellVisibility {
    pub fn new(position: IVec3) -> Self {
        Self {
            visible: false,
            seen: false,
            position,
        }
    }
}
//     pub fn set_seen(&mut self, seen: bool) {
//         self.seen = seen
//     }

//     pub fn set_visible(&mut self, visible: bool) {
//         self.visible = visible
//     }
// }

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
