use crate::AppState;
use bevy::prelude::*;
// use bevy_turborand::prelude::GlobalRng;
use std::collections::HashMap;
use std::ops::Add;

pub mod cartesian {}

#[allow(dead_code)]
const CELL_SIZE_METRES: f32 = 2.0;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Component)]
pub struct Pos2d {
    pub x: i32,
    pub y: i32,
}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Component)]
pub struct Pos3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Pos3d {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn adjacent(self, direction: Direction) -> Pos3d {
        direction.offset() + self
    }
}

impl Add for Pos3d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl From<(i32, i32, i32)> for Pos3d {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Pos3d {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size2d {
    pub width: i32,
    pub height: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size3d {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub origin: Pos2d,
    pub size: Size2d,
}

#[allow(dead_code)]
pub struct RectPrism {
    pub origin: Pos3d,
    pub size: Size3d,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Up,
    Down,
}

// #[derive(Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
// pub enum VerticalDirection {
//     Up,
//     Down,
// }

#[allow(dead_code)]
type Facing = Direction;

#[allow(dead_code)]
impl Direction {
    fn offset(self) -> Pos3d {
        DIRECTION_OFFSETS[self as usize]
    }
}

pub const DIRECTIONS: [Direction; 10] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::Up,
    Direction::Down,
];

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const DIRECTION_OFFSETS: [Pos3d; 10] = [
    Pos3d { x: 0, y: 1, z: 0 },
    Pos3d { x: 1, y: 1, z: 0 },
    Pos3d { x: 1, y: 0, z: 0 },
    Pos3d { x: 1, y: -1, z: 0 },
    Pos3d { x: 0, y: -1, z: 0 },
    Pos3d { x: -1, y: -1, z: 0 },
    Pos3d { x: -1, y: 0, z: 0 },
    Pos3d { x: -1, y: 1, z: 0 },
    Pos3d { x: 0, y: 0, z: 1 },
    Pos3d { x: 0, y: 0, z: -1 },
];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

// ......

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct Board {
    pub size: Size3d,
    pub store: CellStore,
}

#[allow(dead_code)]
impl Board {
    fn default() -> Self {
        Board {
            size: Size3d::default(),
            store: CellStore::default(),
        }
    }

    pub fn set(&mut self, pos: Pos3d, cell: Cell) -> Option<Cell> {
        self.store.cells.insert(pos, cell)
    }

    pub fn get(&self, pos: &Pos3d) -> Option<&Cell> {
        self.store.cells.get(pos)
    }

    pub fn remove(&mut self, pos: &Pos3d) -> Option<Cell> {
        self.store.cells.remove(pos)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Pos3d, &Cell)> {
        self.store.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Pos3d, &mut Cell)> {
        self.store.cells.iter_mut()
    }

    // private

    // fn fill(&mut self, source_cell: &Cell, origin: Pos3d, size: Size3d) {
    //     for x in origin.x..(size.width + origin.x) {
    //         for y in origin.y..(size.height + origin.y) {
    //             for z in origin.z..(size.depth + origin.z) {
    //                 let cell = source_cell.clone();
    //                 let pos = Pos3d { x, y, z };
    //                 self.store.cells.insert(pos, cell);
    //             }
    //         }
    //     }
    // }

    // fn fill_empty(&mut self, origin: Pos3d, size: Size3d) {
    //     for x in origin.x..(size.width + origin.x) {
    //         for y in origin.y..(size.height + origin.y) {
    //             for z in origin.z..(size.depth + origin.z) {
    //                 self.store.cells.insert(Pos3d { x, y, z }, Cell::empty());
    //             }
    //         }
    //     }
    // }

    // fn fill_all(&mut self, source_cell: &Cell) {
    //     self.fill(&source_cell, Pos3d::origin(), self.size)
    // }

    // fn fill_with(&mut self, f: fn(i32, i32, i32) -> Option<Cell>) {
    //     for x in 0..self.size.width {
    //         for y in 0..self.size.height {
    //             for z in 0..self.size.depth {
    //                 if let Some(cell) = f(x, y, z) {
    //                     let pos = Pos3d { x, y, z };
    //                     self.store.cells.insert(pos, cell);
    //                 }
    //             }
    //         }
    //     }
    // }

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
                        self.store.cells.insert(pos, cell);
                    }
                }
            }
        }
    }
}

#[derive(Default, Resource, Eq, PartialEq, Clone, Debug)]
pub struct CellStore {
    cells: HashMap<Pos3d, Cell>,
}

impl CellStore {
    fn default() -> Self {
        CellStore {
            cells: HashMap::new(),
        }
    }
}

//

pub type CellMaterial = Option<Material>;
pub type CellFloor = Option<Material>;
pub type CellItems = Option<Vec<Entity>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Cell {
    pub material: CellMaterial,
    pub floor: CellFloor,
    pub feature: Option<Entity>, // door, trap, statue, well, etc
    pub creature: Option<Entity>,
    pub items: CellItems,
    // fluids, gases, etc
}

impl Cell {
    pub fn empty() -> Self {
        Cell {
            material: None,
            floor: None,
            feature: None,
            creature: None,
            items: Some(vec![]),
        }
    }

    pub fn passable(&self) -> bool {
        match self.material {
            None => true,
            Some(_) => false,
        }
    }
}

impl Default for Cell {
    // A dirt wall
    fn default() -> Self {
        Cell {
            material: Some(Material::Dirt),
            floor: None,
            feature: None,
            creature: None,
            items: Some(vec![]),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::Cell;

//     #[test]
//     fn test_cell_default() {
//         println!("{:?}", Cell::default());
//         assert_ne!(Cell::default(), Cell::empty());
//     }
// }

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

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardRes>()
            .add_systems(OnEnter(AppState::LoadAssets), populate_board);
    }
}

fn populate_board(
    // mut _commands: Commands,
    mut current: ResMut<BoardRes>,
    // mut global_rng: ResMut<GlobalRng>,
) {
    current.board.fill(|x, y, z| {
        if (y % 10 == 0 && x % 6 != 0) || (x % 5 == 0 && y % 3 != 0) {
            Some(Cell::default())
        } else {
            Some(Cell::empty())
        }
    });
}

#[derive(Resource, Debug)]
pub struct BoardRes {
    pub board: Board,
}

impl BoardRes {
    pub fn size(&self) -> &Size3d {
        &self.board.size
    }
}

impl Default for BoardRes {
    fn default() -> Self {
        BoardRes {
            board: Board {
                size: Size3d {
                    width: 48,
                    height: 24,
                    depth: 1,
                },
                ..default()
            },
        }
    }
}
