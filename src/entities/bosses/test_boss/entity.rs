use xf::num::ivec2::IVec2;

use crate::{
    entities::{entity::Entity, data::EntityData}, 
    game::{game_data::GameData, draw_data::DrawData}, 
    graphics::textures::TextureId
};

use super::{test_boss::TestBoss, ai::Ai};



impl Entity for TestBoss {
    fn data(&self) -> &EntityData { &self.d }

    fn data_mut(&mut self) -> &mut EntityData { &mut self.d}

    fn update(&mut self, g: &mut GameData) {
        Ai::update(self, g);
        self.state.update(self, g);

        self.animator.set_key(self.state.to_anim_key(self), false);
        self.animator.update();
    }

    fn draw(&self, d: &mut DrawData, org: IVec2) {
        let texture = d.textures().get(TextureId::TestBoss);
        self.animator.draw(&texture, self.d.pos.as_ivec2() - org, d.buffer());
    }
}