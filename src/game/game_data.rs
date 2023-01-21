use std::mem::replace;

use xf::num::{ivec2::{i2, IVec2}, irect::ir};

use crate::{
    graphics::buffer::Buffer, 
    consts::SCREEN_SIZE, 
    level::{level::Level, load::load_level, level_info::LevelId}, 
    entities::{player::player::Player, bosses::test_boss::test_boss::TestBoss, entity::Entity}
};

use super::draw_data::DrawData;

pub struct GameData {
    pub player: Option<Player>,
    pub boss: Option<TestBoss>,
    pub level: Level,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            level: load_level(LevelId::Test).unwrap(),
            player: Some(Player::new(i2(32, 32))),
            boss: Some(TestBoss::new(i2(128, 32))),
        }
    }

    pub fn update(&mut self) {
        self.update_player();
        self.update_boss();
    }

    fn update_player(&mut self) {
        let mut player = replace(&mut self.player, None);
        if let Some(player) = &mut player {
            player.update(self);
        }
        self.player = player;
    }

    fn update_boss(&mut self) {
        let mut boss = replace(&mut self.boss, None);
        if let Some(boss) = &mut boss {
            boss.update(self);
        }
        self.boss = boss;
    }

    pub fn draw(&self, d: &mut DrawData, org: IVec2) {
        self.level.draw(ir(org, SCREEN_SIZE), d);
        if let Some(player) = &self.player {
            player.draw(d, org);
        }
        if let Some(boss) = &self.boss {
            boss.draw(d, org);
        }

    }
}