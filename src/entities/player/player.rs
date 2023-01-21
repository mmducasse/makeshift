use std::rc::Rc;

use xf::{
    num::{fvec2::FVec2, ivec2::{IVec2, i2}, irect::{IRect, ir, rect}}, 
    data::dir_h::DirH, 
    time::timer::Timer, 
    gl::{texture::Texture, anim::Animator}
};

use crate::{
    entities::{data::EntityData, type_::EntityType}, 
    graphics::textures::TextureId, 
    game::{game_data::GameData, draw_data::DrawData}, consts::P16
};

use super::{state::State, anim::{AnimKey, animator}};

const COLLIDER: IRect = rect(2, 2, 12, 14);


pub struct Player {
    pub d: EntityData,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) state_timer: Timer,
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
            animator: animator(),
        }
    }
}