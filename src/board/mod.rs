use bevy::prelude::{Component, Entity, Resource};
use std::collections::{BTreeMap, HashMap};

pub mod direction;
pub use direction::Direction;

pub mod plugin;
pub use plugin::BoardPlugin;

pub mod primitives;
pub use primitives::*;

// Board
//
#[derive(Clone, Debug, Resource)]
#[allow(dead_code)]
pub struct Board {
    pub size: Size3d,
    pub cells: CellStore,
    pub creatures: CreatureStore,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            size: Size3d {
                width: 48,
                height: 24,
                depth: 1,
            },
            cells: CellStore::default(),
            creatures: CreatureStore::default(),
            // ..default()
        }
    }
}

impl Board {
    pub fn fill<F>(&mut self, f: F)
    where
        F: Fn(i32, i32, i32) -> Option<Cell>,
    {
        self.fill_region(Pos3d::zero(), self.size, f);
    }

    pub fn fill_region<F>(&mut self, origin: Pos3d, size: Size3d, f: F)
    where
        F: Fn(i32, i32, i32) -> Option<Cell>,
    {
        for x in origin.x..(size.width + origin.x) {
            for y in origin.y..(size.height + origin.y) {
                for z in origin.z..(size.depth + origin.z) {
                    if let Some(cell) = f(x, y, z) {
                        let pos = Pos3d { x, y, z };
                        self.cells.set(pos, cell);
                    }
                }
            }
        }
    }
}

// Position
//
#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum Position {
    Area(Area3d),
    Point(Pos3d),
}

// CreatureStore
//

#[derive(Resource, Clone, Debug)]
#[allow(dead_code)]
pub struct CreatureStore {
    // use a BTreeMap here so iter() is ordered by Z,Y,X coordinates
    to_entity: BTreeMap<Pos3d, Entity>,
    // this is the source of truth for the previous mapping
    to_area: HashMap<Entity, Vec<Pos3d>>,
}

impl Default for CreatureStore {
    fn default() -> Self {
        CreatureStore {
            to_entity: BTreeMap::new(),
            to_area: HashMap::new(),
        }
    }
}

pub type Area3d = Vec<Pos3d>;

#[allow(dead_code)]
impl CreatureStore {
    pub fn add(&mut self, entity: Entity, area: Area3d) -> Result<(), &str> {
        if self.to_area.contains_key(&entity) {
            Err("already exists")
        } else {
            self.to_area.insert(entity, area.clone());

            for pos in area.clone() {
                self.to_entity.insert(pos, entity);
            }

            Ok(())
        }
    }

    pub fn add_single(&mut self, entity: Entity, pos: Pos3d) -> Result<(), &str> {
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

    pub fn update_single(&mut self, entity: Entity, pos: Pos3d) -> Result<(), &str> {
        self.update(entity, vec![pos])
    }

    pub fn get_entity_at(&self, pos: &Pos3d) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn get_area_for(&self, entity: &Entity) -> Option<&Area3d> {
        self.to_area.get(entity)
    }

    pub fn get_pos_for(&self, entity: &Entity) -> Option<&Pos3d> {
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

// CellStore
//
#[derive(Resource, Eq, PartialEq, Clone, Debug)]
pub struct CellStore {
    cells: HashMap<Pos3d, Cell>,
}

impl Default for CellStore {
    fn default() -> Self {
        CellStore {
            cells: HashMap::new(),
        }
    }
}

impl CellStore {
    pub fn set(&mut self, pos: Pos3d, cell: Cell) -> Option<Cell> {
        self.cells.insert(pos, cell)
    }

    pub fn get(&self, pos: &Pos3d) -> Option<&Cell> {
        self.cells.get(pos)
    }

    pub fn remove(&mut self, pos: &Pos3d) -> Option<Cell> {
        self.cells.remove(pos)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Pos3d, &Cell)> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Pos3d, &mut Cell)> {
        self.cells.iter_mut()
    }
}

// Cell
//
// a cell is taller than it is wide / deep; about the size a man can stand in.
#[allow(dead_code)]
const CELL_DIMENSIONS_METRES: [f32; 3] = [0.5, 0.5, 2.0];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Component)]
pub struct Cell {
    pub material: CellMaterial,
    pub floor: CellFloor,
    pub feature: Option<Entity>, // door, trap, statue, well, etc
    pub items: CellItems,
    pub visibility: CellVisibility, // to the player: a pragmatic choice
}

impl Cell {}

// type aliases
pub type CellMaterial = Option<Material>;
pub type CellFloor = Option<Material>;
pub type CellItems = Option<Vec<Entity>>;

impl Cell {
    pub fn empty() -> Self {
        Cell {
            material: None,
            floor: None,
            feature: None,
            items: Some(vec![]),
            visibility: CellVisibility::Obscured,
        }
    }

    pub fn passable(&self) -> bool {
        match self.material {
            None => true,
            Some(_) => false,
        }
    }

    pub fn impassable(&self) -> bool {
        !self.passable()
    }
}

impl Default for Cell {
    // A dirt wall
    fn default() -> Self {
        Cell {
            material: Some(Material::Dirt),
            floor: None,
            feature: None,
            items: Some(vec![]),
            visibility: CellVisibility::Obscured,
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

// CellVisibility
//
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CellVisibility {
    Obscured,
    Seen,
    Visible,
}

// CellVisibilityMap
//
#[allow(dead_code)]
#[derive(Component, Clone, Debug)]
pub struct CellVisibilityMap {
    // entity: Entity,
    data: BTreeMap<Pos3d, CellVisibility>,
}
