
use xf::num::{ivec2::{IVec2}, irect::ir, limit::Limit};

use crate::{
    consts::*, 
    level::{level::Level, load::load_level, level_info::LevelId}, 
    entities::{entities::Entities}
};

use super::draw_data::DrawData;

pub struct GameData {
    pub entities: Entities,
    pub level: Level,
    pub player_health: Limit<i32>,
    pub boss_health: Limit<i32>,
}

impl GameData {
    pub fn new() -> Self {
        let (level, entities) = load_level(LevelId::Tunnel).unwrap();

        Self {
            entities,
            level,
            player_health: Limit::new_max(0, PLAYER_HEALTH_MAX),
            boss_health: Limit::new_max(0, BOSS_HEALTH_MAX),
        }
    }

    pub fn update(&mut self) {
        self.level.update();
        Entities::update(self);
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        self.level.draw(ir(org, VIEW_SIZE), d);
        self.entities.draw(d, org);
    }
}