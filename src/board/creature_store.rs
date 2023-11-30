use crate::typical::*;
use bevy::utils::HashMap;
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
        if let bevy::utils::Entry::Vacant(e) = self.to_area.entry(entity) {
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
