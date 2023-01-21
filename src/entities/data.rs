use xf::num::{fvec2::FVec2, ivec2::IVec2, irect::IRect};

use super::{id::EntityId, type_::EntityType};




static mut NEXT_ID: usize = 0;

pub fn next_entity_id() -> usize {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

#[derive(Clone, Copy)]
pub struct EntityData {
    pub id: EntityId,
    pub type_: EntityType,
    pub pos: FVec2,
    pub vel: FVec2,
    pub collider: IRect,
    is_expired: bool,
}

impl EntityData {
    pub fn new(type_: EntityType) -> Self {
        Self {
            id: next_entity_id(),
            type_,
            pos: FVec2::ZERO,
            vel: FVec2::ZERO,
            collider: IRect::ZERO,
            is_expired: false,
        }
    }

    pub fn at(mut self, pos: IVec2) -> Self {
        self.pos = pos.as_fvec2(); self
    }

    pub fn with_collider(mut self, collider: IRect) -> Self {
        self.collider = collider; self
    }

    pub fn expire(&mut self) {
        self.is_expired = true;
    }

    pub fn is_expired(&self) -> bool {
        self.is_expired
    }
}