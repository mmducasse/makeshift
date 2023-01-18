use xf::{
    gl::{bitmap::Bitmap, color::Color}, 
    num::{irect::ir, ivec2::IVec2}
};

use crate::{
    graphics::buffer::buffer_mut, 
    io::controller::get_dir_down, 
};


const COLOR: Color = Color::rgba(0x00FF00FF);

pub struct Object {
    pub pos: IVec2,
}

impl Object {
    pub fn update(&mut self) {
        self.pos += get_dir_down() * 10;
    }

    pub fn draw(&self, org: IVec2) {
        let rect = ir(self.pos, IVec2::splat(20));
        buffer_mut().draw_rect(rect, COLOR)
    }
}