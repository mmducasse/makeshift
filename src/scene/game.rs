use std::process::exit;

use macroquad::{window::next_frame, time::get_fps, prelude::{is_key_down, KeyCode}};
use xf::num::{ivec2::{IVec2}};

use crate::{
    game::{game_data::GameData, draw_data::DrawData, state::GameResult}, 
    consts::*, 
    systems::camera::{Camera}, ui::hud::draw_hud
};


pub async fn run() -> GameResult {

    let mut camera = Camera::new(IVec2::ZERO, VIEW_SIZE);
    let mut g = GameData::new();
    let mut d = DrawData::new();

    let mut i = 0;

    loop {

        if g.player_health.value == 0 {
            return GameResult::Loss;
        }
        if g.boss_health.value == 0 {
            return GameResult::Win;
        }

        g.update();
        camera.update(&g);

        // Debug //////////////////////////////////////////

        if i % 10 == 0 {
            println!("fps = {}", get_fps());
        }
        i += 1;

        if is_key_down(KeyCode::Escape) {
            exit(0);
        }

        // End //////////////////////////////////////////

        g.draw(&mut d, camera.pos());
        draw_hud(HUD_ORIGIN, &g, &mut d);
        
        d.buffer().render();
        next_frame().await;
    }
}