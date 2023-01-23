
use xf::{
    num::{ivec2::{IVec2}, irect::{IRect, rect}}, 
    data::dir_h::DirH, 
    time::{timer::Timer, countdown::Countdown}, 
    gl::{anim::Animator}
};

use crate::{
    entities::{data::EntityData, type_::EntityType},
};

use super::{state::State, anim::{AnimKey, animator}};

const COLLIDER: IRect = rect(2, 2, 12, 14);


pub struct Player {
    pub d: EntityData,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) state_timer: Timer,
    pub(super) grace_timer: Countdown,
    pub animator: Animator<AnimKey>
}

impl Player {
    pub fn new(pos: IVec2) -> Self {
        Self {
            d: EntityData::new(EntityType::Player)
                .at(pos)
                .with_collider(COLLIDER),
            dir: DirH::R,
            state: State::Idle,
            state_timer: Timer::new(0),
            grace_timer: Countdown::new(0),
            animator: animator(),
        }
    }

    pub fn face_dir(&self) -> DirH {
        if matches!(self.state, State::WallSlide) {
            self.dir.opposite()
        } else {
            self.dir
        }
    }
}