// use bevy::window::WindowResolution;

#[doc(inline)]
pub use bevy_ecs_tilemap::helpers::square_grid::neighbors::SquareDirection as Direction;
pub use bevy_ecs_tilemap::map::TilemapSize;
pub use bevy_ecs_tilemap::map::TilemapTileSize;
pub use bevy_ecs_tilemap::prelude::*;
pub use bevy_ecs_tilemap::tiles::TilePos;
pub use logical::*;

pub mod init;
pub mod render;

// pub fn get_tilemap_size(resolution: &WindowResolution, tile_size: &TilemapTileSize) -> TilemapSize {
//     let w: u32 = resolution.width() as u32;
//     let h: u32 = resolution.height() as u32;

//     let x: u32 = w / tile_size.x as u32;
//     let y: u32 = h / tile_size.y as u32;
//     TilemapSize { x, y }
// }

#[allow(dead_code)]
// note:
// we can solve the diagonal movement cost issue with square grids:
// just have them cost more (in terms of time / stamina)
// although this belongs elsewhere
const DIAG_MOVEMENT_COST: f64 = 1.4; // sqrt(2)

use bevy::prelude::Plugin;
use bevy::prelude::Resource;

pub struct MapPlugin {}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_systems(Update, )
        // app.add_state()
        // app.add_event()

        // I think we want to establish a link from GameMap.grid.tilemap to the (equivalent)
        // Bevy_ECS_Tilemap.
        //
        // A few things to think about:
        // - how to support layers
        // - how to support different local maps, and lifecycle of both Grid and TileMap when we
        //   move between local maps
        // - how to sync changes from Grid to TileMap
        // - procedural generation - both implementing it, and where it should live
        // - appropriate lifecycle hooks (systems) to set up all the moving parts
        // - we should probably do very little in this plugin build function

        let mut game_map = GameMap::default();

        game_map
            .grid
            // create a map with all tiles the same
            .clone_cell_to_rect(&Cell::default(), CellPos { x: 0, y: 0 }, game_map.size);

        println!("MapPlugin: generated logical map");
        // println!("{:?}", game_map);

        app.insert_resource(game_map);
    }
}

#[derive(Resource, Debug, Clone)]
#[allow(dead_code)]
struct GameMap {
    grid: Grid,
    size: Size,
    // tilemap_entity: Entity,
}

impl GameMap {
    pub fn new(size: logical::Size) -> Self {
        Self {
            grid: Grid::empty(size),
            size,
        }
    }

    pub fn default() -> Self {
        GameMap::new(Size { x: 64, y: 32 })
    }
}
//

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

type Facing = Direction;

#[allow(dead_code)]
pub mod logical {
    use super::Facing;
    use bevy::prelude::Entity;

    pub type Size = bevy_ecs_tilemap::map::TilemapSize;
    pub type CellPos = bevy_ecs_tilemap::tiles::TilePos;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct CellContents {
        items: Option<()>,
    }
    impl CellContents {
        fn default() -> Self {
            CellContents { items: None }
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub enum Terrain {
        #[default]
        Floor,
        Pillar,
        Wall(Facing),
        Feature(Entity),
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Structure {
        Door(Facing),
        Hatch,
        Trapdoor,
        Altar(Facing),
        Statue(Facing),
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub enum Material {
        #[default]
        Stone,
        Sand,
        Dirt,
        Sandstone,
        Limestone,
        Granite,
        Marble,
        Quartz,
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub enum Fluid {
        #[default]
        Water,
        Brine,
        Muck,
        Blood,
    }

    // #[derive(Clone, Copy)]
    // pub enum ConstructionQuality {
    //     Normal,
    // }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Cell {
        position: CellPos, // is this wise?
        terrain: Terrain,
        contents: CellContents,
        creature: Option<Entity>,
        material: Option<Material>,
        fluid: Option<(Fluid, u8)>,
        trap: Option<Entity>,
    }
    impl Cell {
        fn default() -> Self {
            Cell {
                position: CellPos { x: 0, y: 0 },
                terrain: Terrain::Floor,
                contents: CellContents::default(),
                creature: None,
                material: None,
                fluid: None,
                trap: None,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Grid {
        size: Size,
        storage: Storage,
    }

    impl Grid {
        pub fn empty(size: Size) -> Self {
            let storage = Storage::empty(size);
            Grid { size, storage }
        }

        pub fn clone_cell_to_rect(&mut self, from_cell: &Cell, origin: CellPos, size: Size) {
            for x in origin.x..(size.x + origin.x) {
                for y in origin.y..(size.y + origin.y) {
                    let mut cell = from_cell.clone();
                    cell.position = CellPos { x, y };
                    self.storage.set(&cell.position, cell)
                }
            }
        }

        pub fn clone_cell_to_fill(&mut self, cell: &Cell) {
            self.clone_cell_to_rect(&cell, CellPos { x: 0, y: 0 }, self.size)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Storage {
        cells: Vec<Option<Cell>>,
        size: Size,
    }

    // based closely on TileStorage
    // but contains logical map cells, not texture tiles for rendering

    impl Storage {
        // pub fn build(size: Size) -> Self {
        //     let cells = vec![None; size.count()];
        //     Storage { size, cells }
        //     // populate cells
        // }

        /// Creates a new tile storage that is empty.
        pub fn empty(size: Size) -> Self {
            Self {
                cells: vec![None; size.count()],
                size,
            }
        }

        /// Gets a cell for the given position, if one is associated with that cell
        /// position, or panics if out of bounds.
        pub fn get(self, cell_pos: &CellPos) -> Option<Cell> {
            self.cells[cell_pos.to_index(&self.size)]
        }

        /// Gets a cell for the given cell position, if:
        /// 1) the cell position lies within the underlying map's extents *and*
        /// 2) there is a cell associated with that cell position;
        /// otherwise it returns `None`.
        pub fn checked_get(&self, cell_pos: &CellPos) -> Option<Cell> {
            if cell_pos.within_map_bounds(&self.size) {
                self.cells[cell_pos.to_index(&self.size)]
            } else {
                None
            }
        }

        /// Sets a cell for the given cell position.
        ///
        /// If there is a cell already at that position, it will be replaced.
        ///
        /// Panics if the given `cell_pos` doesn't lie within the extents of
        /// the underlying map.
        pub fn set(&mut self, cell_pos: &CellPos, cell: Cell) {
            self.cells[cell_pos.to_index(&self.size)].replace(cell);
        }

        /// Sets a cell for the given cell position, if the cell position
        /// lies within the
        /// underlying map's extents.
        ///
        /// If there is a cell already at that position, it will be replaced.
        pub fn checked_set(&mut self, cell_pos: &CellPos, cell: Cell) {
            if cell_pos.within_map_bounds(&self.size) {
                // TODO validate the cell's cell_pos
                self.cells[cell_pos.to_index(&self.size)].replace(cell);
            }
        }

        /// Returns an iterator with all of the positions in the grid.
        pub fn iter(&self) -> impl Iterator<Item = &Option<Cell>> {
            self.cells.iter()
        }

        /// Returns mutable iterator with all of the positions in the grid.
        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<Cell>> {
            self.cells.iter_mut()
        }

        /// Remove cell at the given tile position, if there was one, leaving
        /// `None` in its place.
        ///
        /// Panics if the given `cell_pos` doesn't lie within the extents of the underlying cell map.
        pub fn remove(&mut self, cell_pos: &CellPos) {
            self.cells[cell_pos.to_index(&self.size)].take();
        }

        /// Remove any stored cell at the given cell position, if the given
        /// `cell_pos` does lie within the extents of the underlying map.
        ///
        /// Otherwise, nothing is done.
        pub fn checked_remove(&mut self, cell_pos: &CellPos) {
            if cell_pos.within_map_bounds(&self.size) {
                self.cells[cell_pos.to_index(&self.size)].take();
            }
        }
    }
}
