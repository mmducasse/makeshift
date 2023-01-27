use xf::{
    num::{
        ivec2::IVec2, 
        irect::{IRect, rect}
    }, 
    data::dir_h::DirH, 
    gl::anim::Animator, time::countdown::Countdown
};

use crate::entities::{data::EntityData, type_::EntityType, entity::Entity};

use super::{
    state::State, 
    anim::{animator, AnimKey}, 
    ai::Ai, 
};



pub struct Boss {
    pub d: EntityData,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) ai: Ai,
    pub animator: Animator<AnimKey>,
    pub(super) grace_timer: Countdown,
}

const DAMAGE: i32 = 1;
const COLLIDER: IRect = rect(2, 2, 12, 14);

impl Boss {
    pub fn new(pos: IVec2) -> Self {
        Self {
            d: EntityData::new(EntityType::Enemy(DAMAGE))
                .at(pos)
                .with_collider(COLLIDER),
            dir: DirH::L,
            state: State::Idle,
            ai: Ai::new(),
            animator: animator(),
            grace_timer: Countdown::new(0),
        }
    }

    pub fn face_toward(&mut self, x: i32) {
        self.dir = if x < self.bounds().center().x {
            DirH::L
        } else {
            DirH::R
        };
    }
}