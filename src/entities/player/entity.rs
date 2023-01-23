use macroquad::prelude::{KeyCode, is_key_pressed};
use xf::num::{ivec2::{IVec2}, fvec2::f2};

use crate::{entities::{entity::Entity, data::EntityData, spawn::spawn_entity, bullets::bullet::Bullet}, game::{game_data::GameData, draw_data::DrawData}, graphics::textures::TextureId};

use super::player::Player;



impl Entity for Player {
    fn data(&self) -> &EntityData { &self.d }
    fn data_mut(&mut self) -> &mut EntityData { &mut self.d }

    fn update(&mut self, g: &mut GameData) {
        self.state_timer.increment();
        self.grace_timer.decrement();
        self.state.update(self, g);

        // todo delete
        if is_key_pressed(KeyCode::X) {
            let vel_x = self.face_dir().unit().x as f32 * 4.0;
            spawn_entity(Bullet::new(true, self.bounds().center(), f2(vel_x, 0.)), g);
        }

        self.animator.set_key(self.state.to_anim_key(self), false);
        self.animator.update();
    }

    fn draw(&self, d: &mut DrawData, org: IVec2) {
        if self.grace_timer.remaining() % 2 == 0 {
            let texture = d.textures().get(TextureId::Player);
            self.animator.draw(&texture, self.d.pos.as_ivec2() - org, d.buffer());
        }
    }
}