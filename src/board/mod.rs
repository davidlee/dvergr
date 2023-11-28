use bevy::math::UVec3;
use bevy::prelude::{Component, Entity, Resource};
use std::collections::{BTreeMap, HashMap};

pub mod direction;
pub use direction::Direction;

pub mod plugin;
pub use plugin::BoardPlugin;

pub mod primitives;
pub use primitives::*;

pub mod geometry;
pub use geometry::*;

const BOARD_SIZE_W: u32 = 48;
const BOARD_SIZE_H: u32 = 24;
const BOARD_SIZE_Z: u32 = 48;

const BOARD_SIZE_W_USIZE: usize = BOARD_SIZE_W as usize;
const BOARD_SIZE_H_USIZE: usize = BOARD_SIZE_H as usize;
const BOARD_SIZE_Z_USIZE: usize = BOARD_SIZE_Z as usize;

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
                width: BOARD_SIZE_W,
                height: BOARD_SIZE_H,
                depth: 1,
            },
            cells: CellStore::default(),
            creatures: CreatureStore::default(),
        }
    }
}
impl Board {
    pub fn fill<F>(&mut self, f: F)
    where
        F: Fn(u32, u32, u32) -> Option<Cell>,
    {
        self.fill_region(UVec3::new(0, 0, 0), self.size, f);
    }

    pub fn fill_region<F>(&mut self, origin: UVec3, size: Size3d, f: F)
    where
        F: Fn(u32, u32, u32) -> Option<Cell>,
    {
        for x in origin.x..(size.width + origin.x) {
            for y in origin.y..(size.height + origin.y) {
                for z in origin.z..(size.depth + origin.z) {
                    if let Some(cell) = f(x, y, z) {
                        let pos = UVec3 { x, y, z };
                        self.cells.set(pos, cell);
                    }
                }
            }
        }
    }

    pub fn apply_direction(&self, pos: &UVec3, direction: &Direction) -> Result<UVec3, &str> {
        let [x, y, z] = pos.to_array();
        let [ix, iy, iz] = [x as i32, y as i32, z as i32];
        let [dx, dy, dz] = direction.offset().to_array();
        let [x, y, z] = [ix + dx, iy + dy, iz + dz];
        if x < 0 || y < 0 || z < 0 {
            Err("Out of bounds")
        } else {
            let (x, y, z) = (x as u32, y as u32, z as u32);
            if x > self.size.width || y > self.size.height || z > self.size.depth {
                Err("Out of bounds")
            } else {
                Ok(UVec3::new(x as u32, y as u32, z as u32))
            }
        }
    }
}

// Position
//
#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum Position {
    Area(Area3d),
    Point(UVec3),
}

// CreatureStore
//

#[derive(Resource, Clone, Debug)]
#[allow(dead_code)]
pub struct CreatureStore {
    to_entity: HashMap<UVec3, Entity>,
    // this is the source of truth for #to_entity
    to_area: HashMap<Entity, Vec<UVec3>>,
}

impl Default for CreatureStore {
    fn default() -> Self {
        CreatureStore {
            to_entity: HashMap::new(),
            to_area: HashMap::new(),
        }
    }
}

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

    pub fn add_single(&mut self, entity: Entity, pos: UVec3) -> Result<(), &str> {
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

    pub fn update_single(&mut self, entity: Entity, pos: UVec3) -> Result<(), &str> {
        self.update(entity, vec![pos])
    }

    pub fn get_entity_at(&self, pos: &UVec3) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn get_area_for(&self, entity: &Entity) -> Option<&Area3d> {
        self.to_area.get(entity)
    }

    pub fn get_pos_for(&self, entity: &Entity) -> Option<&UVec3> {
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

type CellEntityRow = [Option<Entity>; BOARD_SIZE_W_USIZE];
type CellEntityCol = [Option<Entity>; BOARD_SIZE_H_USIZE];

type Cell2DEntityCol = [CellEntityRow; BOARD_SIZE_H_USIZE];
type Cell2DEntityRow = [CellEntityCol; BOARD_SIZE_W_USIZE];

type Cell3DEntityZYX = [Cell2DEntityCol; BOARD_SIZE_Z_USIZE];
type Cell3DEntityZXY = [Cell2DEntityRow; BOARD_SIZE_Z_USIZE];

#[derive(Resource, Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub struct CellEntityStore {
    zyx: Cell3DEntityZYX,
    zxy: Cell3DEntityZXY,
}

impl Default for CellEntityStore {
    fn default() -> Self {
        CellEntityStore {
            zyx: [[[None; BOARD_SIZE_W_USIZE]; BOARD_SIZE_H_USIZE]; BOARD_SIZE_Z_USIZE],
            zxy: [[[None; BOARD_SIZE_H_USIZE]; BOARD_SIZE_W_USIZE]; BOARD_SIZE_Z_USIZE],
        }
    }
}

impl CellEntityStore {
    fn indices_from_vec(vec: UVec3) -> (usize, usize, usize) {
        let [x, y, z] = vec.to_array();
        (x as usize, y as usize, z as usize)
    }

    pub fn set(&mut self, vec: UVec3, entity: Entity) {
        let (x, y, z) = Self::indices_from_vec(vec);

        self.zyx[z][y][x] = Some(entity);
        self.zxy[z][x][y] = Some(entity);
    }

    pub fn unset(&mut self, vec: UVec3) {
        let (x, y, z) = Self::indices_from_vec(vec);

        self.zyx[z][y][x] = None;
        self.zxy[z][x][y] = None;
    }

    pub fn get(&self, vec: UVec3) -> Option<Entity> {
        let (x, y, z) = Self::indices_from_vec(vec);
        self.zxy[z][x][y]
    }

    pub fn iter_zyx(&self) -> impl Iterator<Item = &Cell2DEntityCol> {
        self.zyx.iter()
    }

    pub fn iter_zxy(&self) -> impl Iterator<Item = &Cell2DEntityRow> {
        self.zxy.iter()
    }

    // can't allow mutation

    // pub fn iter_mut_zyx(&mut self) -> impl Iterator<Item = &mut Cell2DEntityCol> {
    //     self.zyx.iter_mut()
    // }

    // pub fn iter_mut_zxy(&mut self) -> impl Iterator<Item = &mut Cell2DEntityRow> {
    //     self.zxy.iter_mut()
    // }
}

#[derive(Resource, Eq, PartialEq, Clone, Debug)]
pub struct CellStore {
    // TODO make this UVec3, Entity
    cells: HashMap<UVec3, Cell>,
}

impl Default for CellStore {
    fn default() -> Self {
        CellStore {
            cells: HashMap::new(),
        }
    }
}

impl CellStore {
    pub fn set(&mut self, pos: UVec3, cell: Cell) -> Option<Cell> {
        self.cells.insert(pos, cell)
    }

    pub fn get(&self, pos: &UVec3) -> Option<&Cell> {
        self.cells.get(pos)
    }

    pub fn remove(&mut self, pos: &UVec3) -> Option<Cell> {
        self.cells.remove(pos)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&UVec3, &Cell)> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&UVec3, &mut Cell)> {
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
    data: BTreeMap<UVec3, CellVisibility>,
}
