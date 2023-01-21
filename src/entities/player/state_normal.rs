use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, game::game_data::GameData, entities::entity::Entity,
};

use super::{player::Player, state::State, consts::*, state_jump, state_dash};


pub fn start(player: &mut Player, g: &mut GameData) {
    player.state = State::Idle;
}

pub fn update(player: &mut Player, g: &GameData) {
    if let Some(dir) = get_dir_h_down() {
        player.dir = dir;
        player.state = State::Run;
        player.d.vel.x = dir.unit().x as f32 * RUN_SPEED_X;
    } else {
        player.state = State::Idle;
        player.d.vel.x = 0.0;
    }

    if player.state == State::Run &&
       is_key_pressed(DASH_KEY) {
        state_dash::start(player.dir, player);
    }
    else if is_key_pressed(JUMP_KEY) {
        state_jump::start(player);
    }

    player.d.pos += player.d.vel;
    player.d.vel += GRAVITY;

    let deflection = collide(player.bounds(), get_colliders_near(player.bounds().center(), &g), Some(g.level.bounds()));
    player.d.pos += deflection.as_fvec2();

    if player.d.vel.y > 0.0 && deflection.y < 0 {
        player.d.vel.y = 0.0;
    }

    if player.d.vel.y > GRAVITY.y * 4.0 {
        player.state = State::Jump;
    }
}