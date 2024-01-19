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
    to_pos: HashMap<Entity, IVec3>,
}

impl CreatureStore {
    pub fn insert(&mut self, entity: Entity, pos: IVec3) {
        self.to_pos.insert(entity, pos);
        self.to_entity.insert(pos, entity);
    }

    pub fn update(&mut self, entity: Entity, pos: IVec3) {
        self.to_pos.insert(entity, pos);
        self.to_entity.insert(pos, entity);
    }

    pub fn entity_at(&self, pos: &IVec3) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn pos_for(&self, entity: &Entity) -> Option<&IVec3> {
        self.to_pos.get(entity)
    }
}
