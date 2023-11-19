use crate::AppState;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::Add;

pub mod cartesian {}
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Component)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // (Pos {x: 5, y: 10}).adjacent(North) -> Pos {x: 5, y: 11}
    pub fn adjacent(self, direction: Direction) -> Pos {
        direction.offset() + self
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(tuple: (i32, i32)) -> Self {
        Pos::new(tuple.0, tuple.1)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Size {
    width: i32,
    height: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    origin: Pos,
    size: Size,
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
}

type Facing = Direction;

#[allow(dead_code)]
impl Direction {
    fn offset(self) -> Pos {
        OFFSETS[self as usize]
    }
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub const OFFSETS: [Pos; 8] = [
    Pos { x: 0, y: 1 },
    Pos { x: 1, y: 1 },
    Pos { x: 1, y: 0 },
    Pos { x: 1, y: -1 },
    Pos { x: 0, y: -1 },
    Pos { x: -1, y: -1 },
    Pos { x: -1, y: 0 },
    Pos { x: -1, y: 1 },
];

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

// ......

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct Board {
    pub size: Size,
    pub store: CellStore,
}

#[allow(dead_code)]
impl Board {
    fn default() -> Self {
        Board {
            size: Size::default(),
            store: CellStore::default(),
        }
    }

    pub fn set(&mut self, pos: Pos, cell: Cell) -> Option<Cell> {
        self.store.cells.insert(pos, cell)
    }

    pub fn get(&self, pos: &Pos) -> Option<&Cell> {
        self.store.cells.get(pos)
    }

    pub fn remove(&mut self, pos: &Pos) -> Option<Cell> {
        self.store.cells.remove(pos)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Pos, &Cell)> {
        self.store.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Pos, &mut Cell)> {
        self.store.cells.iter_mut()
    }

    // private

    fn clone_cell_to_rect(&mut self, source_cell: &Cell, origin: Pos, size: Size) {
        for x in origin.x..(size.width + origin.x) {
            for y in origin.y..(size.height + origin.y) {
                let cell = source_cell.clone();
                let pos = Pos { x, y };
                self.store.cells.insert(pos, cell);
            }
        }
    }

    fn clone_cell_to_fill(&mut self, cell: &Cell) {
        self.clone_cell_to_rect(&cell, Pos { x: 0, y: 0 }, self.size)
    }
}

#[derive(Default, Resource, Eq, PartialEq, Clone, Debug)]
pub struct CellStore {
    cells: HashMap<Pos, Cell>,
}

impl CellStore {
    fn default() -> Self {
        CellStore {
            cells: HashMap::new(),
        }
    }
}

pub struct BoardPlugin;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Cell {
    kind: CellKind,
    // contents
    // creature
    // material
    // fluid
    // gas
    // trap
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum CellKind {
    #[default]
    Floor,
    Pillar,
    Wall(Facing),
    Door, //(Facing, Entity),
    Feature(Entity),
}

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

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(First, update_board)
            .add_systems(OnEnter(AppState::Game), spawn_map);
    }
}

fn spawn_map(mut _commands: Commands, mut current: ResMut<CurrentBoard>) {
    for x in 0..current.board.size.width {
        for y in 0..current.board.size.height {
            current.board.set(
                Pos { x, y },
                Cell {
                    kind: CellKind::default(),
                },
            );
        }
    }
}

fn update_board(mut _commands: Commands, mut _current: ResMut<CurrentBoard>) {
    // ...
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    board: Board,
}

#[allow(dead_code)]
impl CurrentBoard {
    pub fn size(&self) -> &Size {
        &self.board.size
    }

    // ...
    // fn default() -> Self {
    //     // ..
    //     CurrentBoard {
    //         board: Board::default(),
    //     }
    // }
}
