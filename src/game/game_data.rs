use std::mem::replace;

use xf::num::{ivec2::{i2, IVec2}, irect::ir};

use crate::{
    graphics::buffer::Buffer, 
    consts::SCREEN_SIZE, 
    level::{level::Level, load::load_level, level_info::LevelId}, 
    entities::{player::player::Player, bosses::test_boss::test_boss::TestBoss, entity::Entity, entities::Entities}
};

use super::draw_data::DrawData;

pub struct GameData {
    pub entities: Entities,
    pub level: Level,
}

impl GameData {
    pub fn new() -> Self {
        let (level, entities) = load_level(LevelId::Tunnel).unwrap();

        Self {
            entities,
            level,
        }
    }

    pub fn update(&mut self) {
        self.level.update();
        Entities::update(self);
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        self.level.draw(ir(org, SCREEN_SIZE), d);
        self.entities.draw(d, org);
    }
}