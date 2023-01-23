use xf::{
    gl::{bitmap::Bitmap, color::Color}, 
    num::{irect::{rect}, ivec2::IVec2}
};

use crate::{
    io::controller::get_dir_down, 
    game::{draw_data::DrawData}, 
    graphics::textures::TextureId
};


const COLOR: Color = Color::rgba(0x00FF00FF);

pub struct Object {
    pub pos: IVec2,
}

impl Object {
    pub fn update(&mut self) {
        self.pos += get_dir_down() * 10;
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        let texture = d.textures().get(TextureId::Player);
        let src = rect(0, 0, 16, 16);
        d.buffer().draw_texture(&texture, src, self.pos);
    }
}