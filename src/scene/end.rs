use macroquad::{window::next_frame, prelude::{is_key_pressed, KeyCode}};
use xf::{num::{ivec2::IVec2, irect::IRect}, gl::{bitmap::Bitmap, color::Color}};

use crate::{ui::text::draw_text, game::{state::GameResult, draw_data::DrawData}, consts::SCREEN_SIZE};

pub async fn run(result: GameResult) {

    let mut d = DrawData::new();
    let message = if result == GameResult::Win {
        "You Win!"
    } else { 
        "You Lose :("
    };

    loop {
        if is_key_pressed(KeyCode::Enter) {
            break;
        }

        draw_result_screen(message, &mut d);
        d.buffer().render();
        next_frame().await;
    }
}

fn draw_result_screen(message: &str, d: &mut DrawData) {
    let org: IVec2 = SCREEN_SIZE / 2;
    
    d.buffer().draw_rect(IRect::of_size(SCREEN_SIZE), Color::WHITE);
    draw_text(message, org, true, d);
}