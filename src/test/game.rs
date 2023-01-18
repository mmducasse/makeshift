use macroquad::window::next_frame;
use xf::num::ivec2::{i2, IVec2};

use crate::{
    game::{game_data::GameData, draw_data::DrawData}, 
    graphics::{buffer::Buffer, textures::Textures}, 
    consts::SCREEN_SIZE
};

use super::object::Object;



pub async fn run() {

    let mut g = GameData::new();
    let mut d = DrawData::new();
    let mut object = Object { pos: i2(20, 20) };

    loop {

        object.update();
        object.draw(&mut d, IVec2::ZERO);
        
        d.buffer().render();
        next_frame().await;
    }
}