use macroquad::{texture::Image, prelude::BLUE};
use xf::{
    gl::{bitmap::Bitmap, color::Color, texture::Texture}, 
    num::{ivec2::{i2, IVec2}, irect::IRect}
};

const DEFAULT: Color = Color::WHITE;

/// A wrapper for Macroquad's Image struct
/// that implements xf's Bitmap trait.
pub struct ImageW(Option<Image>);

impl ImageW {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn init(&mut self, screen_size: IVec2) {
        self.0 = Some(
            Image::gen_image_color(screen_size.x as u16, screen_size.y as u16, BLUE)
        );
    }

    pub fn image<'a>(&'a self) -> &'a Image {
        if let Some(image) = &self.0 {
            image
        } else {
            panic!()
        }
    }
}

/// A trait to abstract over any 2D array of pixels.
impl Bitmap for ImageW {
    fn size(&self) -> IVec2 {
        let image = self.image();
        let w = image.width as i32;
        let h = image.height as i32;
        i2(w, h)
    }

    fn get_pixel(&self, pos: IVec2) -> Color {
        let x = pos.x as u32;
        let y = pos.y as u32;
        if let Some(image) = &self.0 {
            let color = image.get_pixel(x, y);
            convert_mq_color_to_xf_color(color)
        } else {
            DEFAULT
        }
    }

    fn set_pixel(&mut self, pos: IVec2, color: Color) {
        if color.a < 120 { return; } //todo: additive color blending.
        
        if IRect::of_size(self.size()).contains(pos) {
            let color = macroquad::color::Color::from_rgba(
                color.r, color.g, color.b, color.a);
        
            let x = pos.x as u32;
            let y = pos.y as u32;
            if let Some(image) = &mut self.0 {
                image.set_pixel(x, y, color);
            }
        }
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