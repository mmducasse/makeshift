use xf::{
    num::{
        ivec2::IVec2, 
        irect::{IRect, rect}
    }, 
    data::dir_h::DirH, 
    gl::anim::Animator, time::countdown::Countdown
};

use crate::entities::{data::EntityData, type_::EntityType};

use super::{
    state::State, 
    anim::{animator, AnimKey}, 
    ai::Ai, consts::JUMP_VEL_Y
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
            dir: DirH::R,
            state: State::Idle,
            ai: Ai::new(),
            animator: animator(),
            grace_timer: Countdown::new(0),
        }
    }

    pub fn jump(&mut self) {
        if self.state != State::Jump {
            self.d.vel.y = JUMP_VEL_Y;
            self.state = State::Jump;
        }
    }
}