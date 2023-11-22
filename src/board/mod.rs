use crate::AppState;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::Add;

pub mod cartesian {}

#[allow(dead_code)]
const CELL_SIZE_METRES: f32 = 2.0;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Component)]
pub struct Pos2d {
    x: i32,
    y: i32,
}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Component)]
pub struct Pos3d {
    x: i32,
    y: i32,
    z: i32,
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

    pub fn origin() -> Self {
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
    width: i32,
    height: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size3d {
    width: i32,
    height: i32,
    depth: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    origin: Pos2d,
    size: Size2d,
}

#[allow(dead_code)]
pub struct RectPrism {
    origin: Pos3d,
    size: Size3d,
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
struct Board {
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

    fn fill(&mut self, source_cell: &Cell, origin: Pos3d, size: Size3d) {
        for x in origin.x..(size.width + origin.x) {
            for y in origin.y..(size.height + origin.y) {
                for z in origin.z..(size.depth + origin.z) {
                    let cell = source_cell.clone();
                    let pos = Pos3d { x, y, z };
                    self.store.cells.insert(pos, cell);
                }
            }
        }
    }

    fn fill_empty(&mut self, origin: Pos3d, size: Size3d) {
        for x in origin.x..(size.width + origin.x) {
            for y in origin.y..(size.height + origin.y) {
                for z in origin.z..(size.depth + origin.z) {
                    self.store.cells.insert(Pos3d { x, y, z }, Cell::empty());
                }
            }
        }
    }

    fn fill_all(&mut self, source_cell: &Cell) {
        self.fill(&source_cell, Pos3d::origin(), self.size)
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default)]
pub struct Cell {
    material: CellMaterial,
    floor: CellFloor,
    feature: Option<Entity>, // door, trap, statue, well, etc
    creature: Option<Entity>,
    items: CellItems,
    // fluids, gases, etc
}

impl Cell {
    pub fn empty() -> Self {
        Cell { ..default() }
    }
}

#[derive(Default, Resource, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum Material {
    Dirt,
    #[default]
    Sandstone,
    Granite,
    Marble,
    Quartz,
    Sand,
}

// pub struct MaterialAttrs {
//     hardness: i32,
//     digs_into: Option<Material>,
// }
//
// #[derive(Default, Resource, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
// pub enum CellMaterial {
//     #[default]
//     Empty,
//     Solid(Material), // ...
// }

// #[derive(Default, Resource, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
// pub enum CellFloor {
//     #[default]
//     None,
//     Solid(Material),
// }

// #[derive(Default, Resource, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
// pub enum CellItems {
//     #[default]
//     Empty,
//     List(Vec<Entity>),
// }
// #[derive(Clone, Copy, Debug, Default)]
// pub enum Material {
//     #[default]
//     Stone,
//     Sand,
//     Dirt,
//     Sandstone,
//     Limestone,
//     Granite,
//     Marble,
//     Quartz,
// }

// #[derive(Clone, Copy, Debug, Default)]
// pub enum Fluid {
//     #[default]
//     Water,
//     Brine,
//     Muck,
//     Blood,
// }

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            // .add_systems(First, update_board)
            .add_systems(OnEnter(AppState::Game), spawn_map);
    }
}

fn spawn_map(mut _commands: Commands, mut current: ResMut<CurrentBoard>) {
    let size = current.size().clone();
    current.board.fill_empty(Pos3d::origin(), size);
}

// fn update_board(mut _commands: Commands, mut current: ResMut<CurrentBoard>) {
//     // ...
// }

#[derive(Default, Resource)]
pub struct CurrentBoard {
    board: Board,
}

impl CurrentBoard {
    pub fn size(&self) -> &Size3d {
        &self.board.size
    }
}
