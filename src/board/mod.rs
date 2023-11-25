// use crate::creature::Creature;
use crate::state::AppState;
use bevy::prelude::{Entity, Resource};
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
    // Cells ..

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

    // Creatures ...

    // pub fn insert_creature_pos<'a>(&mut self, cp: CreaturePos) -> Result<(), &str> {
    //     // check if cell(s) are valid
    //     if self.is_valid_creature_pos(&cp) {
    //         Ok(self.creatures.v.push(cp))
    //     } else {
    //         Err("invalid creature position")
    //     }
    // }

    // pub fn is_valid_creature_pos(&self, cp: &CreaturePos) -> bool {
    //     println!("not implemented: is valid cp? {:?}", cp);
    //     true // FIXME
    // }

    // pub fn move_creature<'a, 'b>(
    //     &self,
    //     // from_pos: &Pos3d,
    //     // to_pos: &'a Pos3d,
    // ) -> Result<&'a Creature, &'b str> {
    //     // let mut from_cell = self.get(from_pos).unwrap();
    //     // let mut creature: Creature;

    //     // match from_cell.creature {
    //     //     Some(creature) => println!("got a {:?}", creature),
    //     //     None => panic!("No creature found"),
    //     // }

    //     // match self.get(to_pos) {
    //     //     Some(to_cell) => {
    //     //         if to_cell.impassable() {
    //     //             Err("position impassable")
    //     //         } else if to_cell.occupied() {
    //     //             Err("position occupied")
    //     //         } else {
    //     //             // move the creature and return it
    //     //             // first remove it from the
    //     //             from_cell.creature = None;
    //     //             to_cell.creature = Some(creature);
    //     //             creature.position = to_pos.clone();
    //     //             Ok(&creature)
    //     //         }
    //     //     }
    //     //     None => Err("Invalid position"),
    //     // }
    //     Err("not implemented")
    // }
}
/*
// CreatureStore:

 - allow querying of Board position(s) to find any Creature(s) there
 - allow finding the Board position(s) occupied by a particular Creature
 - allow Creatures to also exist in vanilla ECS (i.e not holding references exclusively here)
   - this implies storing Entities, and using `query.get` to return the Creature components
   - for now, a (logical) Creature's Entity doesn't need to have a parent Entity
 - support a Creature occupying 1 or any number of cells, depending on
   size, stance, actions, etc
   - a human warrior might occupy 1, 2 or 4 cells
   - a swarm might occupy many non-exclusively, may be irregular and non-contiguous
   - but _usually_ a combatant will exclusively occupy a cell

# data structure options:
## 1. BTreeMap<Pos3d, Entity>

+ allows one Creature to occupy arbitrary cells (multiple entries)
+ iterate over cells will remain sorted
+ sparse set, good perf

+ easy to find Creature for given cell
- easy to accidentally remove Creature from all cells
- hard to find all cells for a given Creature

## 2. Vec<(Entity, Vec<Pos3d>)>:

+ easy to get all cells for entity
+ simple
+ easy to validate correct # of cells / avoid orphans
+ perf likely fine

- hard to find whether a given cell is occupied

## 3. Composite Data Structure
### a. BTreeMap<Pos3d, CreaturePos> + component CreaturePos { entity, Vec<Pos3d> }

where we have multiple refs in the BTree pointing to one struct with the canonical list of cells,
and the entity.

+ easy to get the list of cells / locations given the entity
+ easy to get / check for the entity given a cell / location
+ supports multiple cells per creature

- more complex
- more than one copy of the data, requires keeping in sync (although CreaturePos is the SoT)
- may be a pain having to access via a Query

### b. CreatureStore { to_entity, to_pos }

+ most of the above
+ it seems simpler
+ one struct should be able to keep itself in sync

*/

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

type Area = Vec<Pos3d>;

#[allow(dead_code)]
impl CreatureStore {
    pub fn add(&mut self, entity: Entity, area: Area) -> Result<(), &str> {
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

    pub fn update(&mut self, entity: Entity, area: Area) -> Result<(), &str> {
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

    pub fn get_entity_at(&self, pos: &Pos3d) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn get_area_for(&self, entity: &Entity) -> Option<&Area> {
        self.to_area.get(entity)
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Cell {
    pub material: CellMaterial,
    pub floor: CellFloor,
    pub feature: Option<Entity>, // door, trap, statue, well, etc
    pub items: CellItems,
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
            // creature: None,
            items: Some(vec![]),
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

    // pub fn occupied(&self) -> bool {
    //     false
    //     // match self.creature {
    //     //     None => false,
    //     //     Some(_) => true,
    //     // }
    // }

    // pub fn unoccupied(&self) -> bool {
    //     !self.occupied()
    // }
}

impl Default for Cell {
    // A dirt wall
    fn default() -> Self {
        Cell {
            material: Some(Material::Dirt),
            floor: None,
            feature: None,
            items: Some(vec![]),
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
