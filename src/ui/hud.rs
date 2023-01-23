use std::fmt::format;

use xf::{num::{ivec2::{IVec2, i2}, irect::IRect}, gl::{bitmap::Bitmap, color::Color}};

use crate::{game::{game_data::GameData, draw_data::DrawData}, consts::HUD_SIZE};

use super::text::draw_text;



pub fn draw_hud(org: IVec2, g: &GameData, d: &mut DrawData) {
    d.buffer().draw_rect(IRect::of_size(HUD_SIZE).offset_by(org), Color::WHITE);

    let player_str = format!("Player: {}", 123);
    const PLAYER_TEXT_ORG: IVec2 = i2(8, 8);
    draw_text(&player_str, PLAYER_TEXT_ORG + org, d);

    let boss_str = format!("Boss: {}", 123);
    const BOSS_TEXT_ORG: IVec2 = i2(128, 8);
    draw_text(&boss_str, BOSS_TEXT_ORG + org, d);
}