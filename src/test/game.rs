use macroquad::window::next_frame;
use xf::num::ivec2::{i2, IVec2};

use crate::graphics::buffer::render_buffer;

use super::object::Object;



pub async fn run() {

    let mut object = Object { pos: i2(20, 20) };

    loop {

        object.update();
        object.draw(IVec2::ZERO);
        
        render_buffer(false, 1.0);
        next_frame().await;
    }
}