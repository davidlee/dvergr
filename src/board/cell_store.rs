use crate::typical::*;
use bevy::utils::HashMap;

// CellStore
//
#[derive(Resource, Eq, PartialEq, Clone, Debug, Default)]
pub(crate) struct EntityPositionStore {
    // TODO make this IVec3, Entity
    to_entity: HashMap<IVec3, Entity>,
    to_uvec: HashMap<Entity, IVec3>,
}

#[allow(dead_code)]
impl EntityPositionStore {
    pub fn as_hashset2d(&self) -> HashSet<[i32; 2]> {
        self.to_entity.keys().fold(HashSet::new(), |mut acc, pos| {
            acc.insert([pos.x, pos.y]);
            acc
        })
    }

    pub fn set(&mut self, pos: IVec3, entity: Entity) {
        self.to_entity.insert(pos, entity);
        self.to_uvec.insert(entity, pos);
    }

    pub fn get(&self, pos: &IVec3) -> Option<&Entity> {
        self.to_entity.get(pos)
    }

    pub fn get_pos(&self, entity: &Entity) -> Option<&IVec3> {
        self.to_uvec.get(entity)
    }

    pub fn remove(&mut self, pos: &IVec3) -> Option<Entity> {
        if let Some(entity) = self.to_entity.remove(pos) {
            self.to_uvec.remove(&entity);
            Some(entity)
        } else {
            None
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Option<Entity> {
        if let Some(pos) = self.to_uvec.remove(&entity) {
            self.to_entity.remove(&pos);
            Some(entity)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IVec3, &Entity)> {
        self.to_entity.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&IVec3, &mut Entity)> {
        self.to_entity.iter_mut()
    }
}
