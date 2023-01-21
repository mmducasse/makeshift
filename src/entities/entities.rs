use std::{mem::replace, ops::Deref, f32::consts::E};

use xf::num::ivec2::IVec2;

use crate::game::{draw_data::DrawData, game_data::GameData};

use super::{
    entity::Entity, 
    player::player::Player, 
    bosses::test_boss::test_boss::TestBoss, type_::EntityType
};



pub struct Entities {
    pub all: Vec<Option<Box<dyn Entity>>>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            all: vec![],
        }
    }

    pub fn iter(&self) -> EntitiesIterator {
        EntitiesIterator {
            entities: &self,
            idx: 0,
        }
    }

    pub fn get(&self, type_: EntityType) -> Option<&dyn Entity> {
        self.iter().filter(|e| e.data().type_ == type_).nth(0)
    }

    pub fn add(&mut self, entity: Box<dyn Entity>) {
        for idx in 0..self.all.len() {
            if self.all[idx].is_none() {
                self.all[idx] = Some(entity);
                return;
            }
        }

        self.all.push(Some(entity));
    }

    pub fn update(g: &mut GameData) {
        // Pull out each entity to give them a mutable reference to g while updating.
        for idx in 0..g.entities.all.len() {
            let mut entity = replace(&mut g.entities.all[idx], None);

            if let Some(entity) = &mut entity {
                entity.update(g);
            }

            g.entities.all[idx] = entity;
        }

        // Remove all expired entities.
        for idx in 0..g.entities.all.len() {
            let mut delete = false;
            if let Some(entity) = &g.entities.all[idx] {
                delete = entity.data().is_expired();
            }
            if delete {
                g.entities.all[idx] = None;
            }
        }
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        for e in self.iter() {
            e.draw(d, org);
        }
    }
}

pub struct EntitiesIterator<'a> {
    pub entities: &'a Entities,
    idx: usize,
}

impl<'a> Iterator for EntitiesIterator<'a> {
    type Item = &'a dyn Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.entities.all.len() { return None }
        while self.entities.all[self.idx].is_none() {
            self.idx += 1;
            if self.idx >= self.entities.all.len() { return None }
        }

        if let Some(entity) = &self.entities.all[self.idx] {
            self.idx += 1;
            let x = entity.deref();
            Some(x)
        } else {
            None
        }
    }
}