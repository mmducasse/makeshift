use macroquad::shapes::draw_rectangle;
use xf::{num::{ivec2::{IVec2, i2}, irect::ir}, gl::{bitmap::Bitmap, color::Color}};

use crate::{game::{game_data::GameData, draw_data::DrawData}, consts::{P16, SCREEN_SIZE}, graphics::textures::TextureId};

const LAYER_1_X_SPEED: f32 = 1.0;
const LAYER_2_X_SPEED: f32 = 0.5;

const LAYER_X_SIZE: i32 = 32 * P16.x;

const BG_COLOR: Color = Color::rgba(0x93A5F7FF);

pub struct Background {
    pub layer_1_x_pos: f32,
    pub layer_2_x_pos: f32,
}

impl Background {
    pub fn new() -> Self {
        Self {
            layer_1_x_pos: 0.0,
            layer_2_x_pos: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.layer_1_x_pos -= LAYER_1_X_SPEED;

        if self.layer_1_x_pos < -LAYER_X_SIZE as f32 {
            self.layer_1_x_pos += LAYER_X_SIZE as f32;
        }

        self.layer_2_x_pos -= LAYER_2_X_SPEED;

        if self.layer_2_x_pos < -LAYER_X_SIZE as f32 {
            self.layer_2_x_pos += LAYER_X_SIZE as f32;
        }
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        // Draw blue background.
        let rect_size = i2(SCREEN_SIZE.x, 10 * P16.y);
        d.buffer().draw_rect(ir(IVec2::ZERO, rect_size), BG_COLOR);

        // Draw layer 2 clouds.
        let texture = d.textures().get(TextureId::CloudLayer2);
        d.buffer().draw_texture_full(&texture, i2(self.layer_2_x_pos as i32, 0));
        d.buffer().draw_texture_full(&texture, i2(self.layer_2_x_pos as i32 + LAYER_X_SIZE, 0));

        // Draw layer 1 clouds.
        let texture = d.textures().get(TextureId::CloudLayer1);
        d.buffer().draw_texture_full(&texture, i2(self.layer_1_x_pos as i32, 0));
        d.buffer().draw_texture_full(&texture, i2(self.layer_1_x_pos as i32 + LAYER_X_SIZE, 0));
    }
}