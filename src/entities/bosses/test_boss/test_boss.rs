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
    consts::JUMP_VEL_Y, 
    ai::Ai, 
};



pub struct TestBoss {
    pub d: EntityData,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) ai: Ai,
    pub animator: Animator<AnimKey>,
    pub(super) grace_timer: Countdown,
}

const DAMAGE: i32 = 1;
const COLLIDER: IRect = rect(2, 2, 12, 14);

impl TestBoss {
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

    pub fn jump(&mut self) {
        if self.state != State::Jump {
            self.d.vel.y = JUMP_VEL_Y;
            self.state = State::Jump;
        }
    }
}