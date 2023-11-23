# design goals

- sufficient performance (not optimising for max perf)
  - simulation performance > representation performance
- decoupling between domain data & logic, and representation
  - MODEL vs RENDER
  - Model & Render should know nothing of each other
  - Model -> Mapper -> Renderer
  - Mapper takes domain events / data and determines how to map them to rendering requirements, eg.
    - given a cell with given terrain type and material, the renderer should use this texture.  
  - Information should flow from Model -> Mapper -> Renderer
    - potential violations? 
      - wait for animation
      - ...
  - Events should be used for coordination
  - Global locking should be avoided where reasonably possible
- aspirationally: should be able to support 3D with acceptable performance
  - line of sight
  - terrain destruction
  - maybe voxel gfx?
  - fluid dynamics

- approaches
  - parent/child relationship: Model owns Renderer
    - higher coupling
    - transform / translate may interfere
    - I don't like it
  - separate systems
    - must be kept in sync
    - Model updates should precede Render updates in each frame
    - potential challenges w. dangling references, eg. 
      - if Model Entities are deleted;
      - detecting if Renderable instances are removed / dropped
      - surely events work though?
      - and .. just don't delete Entities?
  

- questions
 - should the Model use an Entity / Component per tile; or, operate on a Resource collection?
 - what's the best way to mark cells which require update? 
   - Are there separate classes of these, requiring different Components for effective querying?
   - eg creature cell movement, terrain damage, combat animation
 - should it account for mobs & items, or just map tiles? 




...


RES.grid.cells[ (x y z )] -> Cell { kind, }
    
   

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
