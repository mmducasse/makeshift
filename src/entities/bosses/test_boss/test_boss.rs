use xf::{num::{fvec2::FVec2, ivec2::{IVec2, i2}, irect::{IRect, ir}}, data::dir_h::DirH, gl::anim::Animator};

use crate::{entities::entity::next_entity_id, game::{game_data::GameData, draw_data::DrawData}, graphics::textures::TextureId};

use super::{state::State, anim::{animator, AnimKey}, ai::Ai, consts::JUMP_VEL_Y};



pub struct TestBoss {
    pub id: usize,
    pub pos: FVec2,
    pub vel: FVec2,
    pub dir: DirH,
    pub(super) state: State,
    pub(super) ai: Ai,
    animator: Animator<AnimKey>,
}


impl TestBoss {
    pub fn new(pos: IVec2) -> Self {
        Self {
            id: next_entity_id(),
            pos: pos.as_fvec2(),
            vel: FVec2::ZERO,
            dir: DirH::R,
            state: State::Idle,
            ai: Ai::new(),
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

    pub fn jump(&mut self) {
        if self.state != State::Jump {
            self.vel.y = JUMP_VEL_Y;
            self.state = State::Jump;
        }
    }

    pub fn update(&mut self, g: &mut GameData) {
        Ai::update(self, g);
        self.state.update(self, g);

        self.animator.set_key(self.state.to_anim_key(self), false);
        self.animator.update();
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        let texture = d.textures().get(TextureId::TestBoss);
        self.animator.draw(&texture, self.pos.as_ivec2() - org, d.buffer());
    }
}