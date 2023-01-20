use macroquad::window::next_frame;
use xf::num::{ivec2::{i2, IVec2}, irect::ir};

use crate::{
    game::{game_data::GameData, draw_data::DrawData}, 
    graphics::{buffer::Buffer, textures::Textures}, 
    consts::SCREEN_SIZE, 
    level::{load::load_level, level_info::LevelId}, 
    entities::player::player::Player, 
    systems::camera::{self, Camera}
};

use super::object::Object;



pub async fn run() {

    let mut camera = Camera::new(IVec2::ZERO, SCREEN_SIZE);
    let mut g = GameData::new();
    let mut d = DrawData::new();
    let mut player = Player::new(i2(20, 20));

    loop {

        player.update(&mut g);
        camera.update(player.bounds(), g.level.bounds());

        g.level.draw(ir(camera.pos(), SCREEN_SIZE), &mut d);
        player.draw(&mut d, camera.pos());
        
        d.buffer().render();
        next_frame().await;
    }
}