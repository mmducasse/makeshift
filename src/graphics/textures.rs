use std::{rc::Rc, collections::HashMap};

use xf::gl::texture::Texture;

use super::buffer::xf_texture_from_bytes;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum TextureId {
    Player,
}

const COUNT: usize = 7;

const fn get_bytes(id: TextureId) -> &'static [u8] {
    use TextureId::*;

    match id {
        Player => include_bytes!("../../assets/Player.png"),
    }
}

pub struct Textures {
    map: HashMap<TextureId, Rc<Texture>>,
}

impl Textures {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Get's the `Texture` associated with the `TextureId`.
    /// Loads the texture if it isn't already loaded.
    pub fn get(&mut self, id: TextureId) -> Rc<Texture> {
        if !self.map.contains_key(&id) {
            self.load(id);
        }

        self.map[&id].clone()
    }

    fn load(&mut self, id: TextureId) {
        let bytes = get_bytes(id);
        let texture = xf_texture_from_bytes(bytes);
        self.map.insert(id, Rc::new(texture));
    }
}