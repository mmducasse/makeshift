use xf::num::ivec2::IVec2;

use crate::{
    entities::{entity::Entity, data::EntityData}, 
    game::{game_data::GameData, draw_data::DrawData}, 
    graphics::textures::TextureId, systems::collision::is_wall_at
};

use super::bullet::Bullet;


impl Entity for Bullet {
    fn data(&self) -> &EntityData { &self.d }

    fn data_mut(&mut self) -> &mut EntityData { &mut self.d}

    fn update(&mut self, g: &mut GameData) {
        self.animator.update();
        self.d.pos += self.d.vel;

        if !g.level.bounds().contains(self.bounds().center()) ||
           is_wall_at(self.bounds(), IVec2::ZERO, g)
        {
            self.d.expire();
        }
    }

    fn draw(&self, d: &mut DrawData, org: IVec2) {
        let texture = d.textures().get(TextureId::Bullets);
        self.animator.draw(&texture, self.d.pos.as_ivec2() - org, d.buffer());
    }
}