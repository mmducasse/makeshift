use macroquad::window::next_frame;
use xf::num::{ivec2::{i2, IVec2}, irect::ir};

use crate::{
    game::{game_data::GameData, draw_data::DrawData}, 
    graphics::{buffer::Buffer, textures::Textures}, 
    consts::SCREEN_SIZE, 
    level::{load::load_level, level_info::LevelId}, 
    entities::{player::player::Player, spawn::spawn_entity, bosses::test_boss::test_boss::TestBoss}, 
    systems::camera::{self, Camera}
};

use super::object::Object;



pub async fn run() {

    let mut camera = Camera::new(IVec2::ZERO, SCREEN_SIZE);
    let mut g = GameData::new();
    let mut d = DrawData::new();

    spawn_entity(Player::new(i2(32, 32)), &mut g);
    spawn_entity(TestBoss::new(i2(128, 32)), &mut g);

    loop {

        g.update();
        camera.update(&g);

        // Debug

        //println!("count = {}", g.entities.debug_count());

        // End

        g.draw(&mut d, camera.pos());
        
        d.buffer().render();
        next_frame().await;
    }
}