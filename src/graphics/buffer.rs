use macroquad::{
    texture::{Image, Texture2D, FilterMode, draw_texture_ex, DrawTextureParams}, 
    prelude::{BLUE, WHITE, Vec2}, 
    window::{screen_width, screen_height, clear_background}
};

use xf::{
    gl::{bitmap::Bitmap, color::Color, texture::Texture}, 
    num::{ivec2::{i2, IVec2}, irect::{IRect, ir}}
};

const DEFAULT: Color = Color::WHITE;

/// The back buffer for drawing.
/// Really a wrapper for Macroquad's Image struct
/// that implements xf's Bitmap trait.
pub struct Buffer(Image);

impl Buffer {
    pub fn new(screen_size: IVec2) -> Self {
        Self(
            Image::gen_image_color(screen_size.x as u16, screen_size.y as u16, BLUE)
        )
    }

    pub fn image(&self) -> &Image {
        &self.0
    }

    pub fn render(&mut self) {
        // Convert the buffer to a Texture2D then draw it (not very efficient...).
        let texture2d = Texture2D::from_image(self.image());
        texture2d.set_filter(FilterMode::Nearest);

        clear_background(macroquad::prelude::YELLOW); // todo delete

        draw_texture_ex(texture2d, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        });

        // Clear the buffer.
        // let rect = ir(IVec2::ZERO, SCREEN_SIZE);
        // self.draw_rect(rect, Color::BLACK);

    }
}

/// A trait to abstract over any 2D array of pixels.
impl Bitmap for Buffer {
    fn size(&self) -> IVec2 {
        let image = self.image();
        let w = image.width as i32;
        let h = image.height as i32;
        i2(w, h)
    }

    fn get_pixel(&self, pos: IVec2) -> Color {
        let x = pos.x as u32;
        let y = pos.y as u32;
        let color = self.image().get_pixel(x, y);
        convert_mq_color_to_xf_color(color)
    }

    unsafe fn set_pixel(&mut self, pos: IVec2, color: Color) {
        if color.a < 120 { return; } //todo: additive color blending.
        
        let color = macroquad::color::Color::from_rgba(
            color.r, color.g, color.b, color.a);
    
        let x = pos.x as u32;
        let y = pos.y as u32;
        self.0.set_pixel(x, y, color);
    }
}

pub fn convert_mq_color_to_xf_color(color: macroquad::prelude::Color) -> Color {
    let bs: [u8; 4] = color.into();
    Color {
        r: bs[0],
        g: bs[1],
        b: bs[2],
        a: bs[3],
    }
}

pub fn convert_mq_image_to_xf_texture(image: &Image) -> Texture {
    let size = i2(image.width as i32, image.height as i32);
    let mut texture = Texture::of_size(size);

    for pos in texture.bounds().iter() {
        let x = pos.x as u32;
        let y = pos.y as u32;
        let color = image.get_pixel(x, y);
            
        texture.set(pos, convert_mq_color_to_xf_color(color));
    }

    texture
}

pub fn xf_texture_from_bytes(bytes: &'static [u8]) -> Texture {
    let image = Image::from_file_with_format(bytes, None);
    convert_mq_image_to_xf_texture(&image)
}