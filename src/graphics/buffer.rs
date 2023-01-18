use macroquad::{
    texture::{Texture2D, FilterMode, draw_texture_ex, DrawTextureParams}, 
    prelude::{WHITE, Vec2}, 
    window::{screen_width, screen_height}
};
use xf::{
    num::{ivec2::IVec2, irect::{ir, IRect}}, 
    gl::{
        color::Color, 
        texture::Texture, 
        draw_params::DrawParams, 
        bitmap::Bitmap
    }
};

use crate::consts::SCREEN_SIZE;

use super::image::ImageW;

/// The pixel buffer that gets drawn to the screen at the end of each frame cycle.
static mut BUFFER: ImageW = ImageW::new();

pub fn buffer() -> &'static ImageW {
    unsafe { &BUFFER }
}

pub fn buffer_mut() -> &'static mut ImageW {
    unsafe { &mut BUFFER }
}

/// Draws the buffer to the screen then clears the buffer.
pub fn render_buffer(flip_x: bool, x_scale: f32) {
    unsafe {
        // Convert the buffer to a Texture2D then draw it (not very efficient...).
        let texture2d = Texture2D::from_image(BUFFER.image());
        texture2d.set_filter(FilterMode::Nearest);

        let w = screen_width() * x_scale;
        let h = screen_height();
        let x = (screen_width() - w) / 2.0;

        draw_texture_ex(texture2d, x, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(w, h)),
            source: None,
            rotation: 0.0,
            flip_x,
            flip_y: false,
            pivot: None,
        });

        // Clear the buffer.
        const BLACK: Color = Color::rgba(0x0000_00FF);
        let rect = ir(IVec2::ZERO, SCREEN_SIZE);
        BUFFER.draw_rect(rect, BLACK);
    }
}

pub fn draw_rect(rect: IRect, color: Color) {
    buffer_mut().draw_rect(rect, color);
}

pub fn draw_texture(texture: &Texture, src: IRect, dst_pt: IVec2) {
    buffer_mut().draw_texture(texture, src, dst_pt);
}

pub fn draw_texture_full(texture: &Texture, dst_pt: IVec2) {
    buffer_mut().draw_texture_full(texture, dst_pt);
}

pub fn draw_texture_x(texture: &Texture, dst_pt: IVec2, params: DrawParams) {
    buffer_mut().draw_texture_x(texture, dst_pt, params);
}