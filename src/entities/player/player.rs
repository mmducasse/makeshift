use std::rc::Rc;

use xf::{
    num::{fvec2::FVec2, ivec2::{IVec2, i2}, irect::{IRect, ir}}, 
    data::dir_h::DirH, 
    time::timer::Timer, 
    gl::{texture::Texture, anim::Animator}
};

use crate::{entities::entity::next_entity_id, graphics::textures::TextureId, game::{game_data::GameData, draw_data::DrawData}};

use super::{state::State, anim::{AnimKey, animator}, update_data::PlayerUpdateData};


pub struct Player {
    id: usize,
    pub pos: FVec2,
    pub vel: FVec2,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) state_timer: Timer,
    //texture: Rc<Texture>,
    animator: Animator<AnimKey>
}

impl Player {
    pub fn new(pos: IVec2) -> Self {
        Self {
            id: next_entity_id(),
            pos: pos.as_fvec2(),
            vel: FVec2::ZERO,
            dir: DirH::R,
            state: State::Idle,
            state_timer: Timer::new(0),
            animator: animator(),
        }
    }

    pub fn bounds(&self) -> IRect {
        ir(
            self.pos.as_ivec2() + i2(2, 2), 
            i2(12, 14)
        )
    }

    pub fn anchor(&self) -> IVec2 {
        self.bounds().center() + i2(0, 4)
    }

    pub fn update(&mut self, g: &mut GameData) {
        self.state_timer.increment();
        self.state.update(self, &mut PlayerUpdateData { 
            game_data: g,
        });

        // let touch_tile = d.level.tile_type_at(self.anchor(), d.scene_state);
        // if !Self::can_touch(touch_tile, &d.global_state) {
        //     self.pos = d.global_state.last_checkpoint_pos;
        //     self.vel = FVec2::ZERO;
        // }

        self.animator.set_key(self.state.to_anim_key(self), false);
        self.animator.update();
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        let texture = d.textures().get(TextureId::Player);
        self.animator.draw(&texture, self.pos.as_ivec2() - org, d.buffer());
    }
}