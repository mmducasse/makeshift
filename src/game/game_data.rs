use crate::{
    graphics::buffer::Buffer, 
    consts::SCREEN_SIZE, 
    level::{level::Level, load::load_level, level_info::LevelId}
};

pub struct GameData {
    pub level: Level,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            level: load_level(LevelId::Test).unwrap(),
        }
    }
}