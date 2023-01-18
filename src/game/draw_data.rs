use xf::{gl::bitmap::Bitmap, num::ivec2::IVec2};

use crate::{
    graphics::{textures::Textures, buffer::Buffer}, 
    consts::SCREEN_SIZE
};



pub struct DrawData {
    textures: Textures,
    buffer: Buffer,
}

impl DrawData {
    pub fn new() -> Self {
        Self {
            textures: Textures::new(),
            buffer: Buffer::new(SCREEN_SIZE),
        }
    }

    pub fn textures(&mut self) -> &mut Textures {
        &mut self.textures
    }

    pub fn buffer(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}