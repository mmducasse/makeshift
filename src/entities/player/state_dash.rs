use macroquad::prelude::{is_key_pressed, KeyCode, is_key_down};
use xf::{time::timer::Timer, data::dir_h::DirH};

use crate::{
    systems::collision::collide, 
    consts::GRAVITY, 
};

use super::{
    player::Player, 
    state::State, 
    consts::*, 
    state_jump, 
    update_data::PlayerUpdateData, 
    state_normal
};

pub fn start(dir: DirH, player: &mut Player) {
    player.dir = dir;
    player.state = State::Dash;
    player.state_timer = Timer::new(DASH_TIME_S);
    player.vel.x = dir.unit().x as f32 * DASH_SPEED_X;
    //global::player_state::get_mut().last_checkpoint_pos = player.pos;
}

pub fn update(player: &mut Player, d: &mut PlayerUpdateData) {
    if !is_key_down(DASH_KEY) ||
       player.state_timer.is_done() {
        state_normal::start(player, d);
    }

    if is_key_pressed(JUMP_KEY) {
        state_jump::start(player);
    }

    player.pos += player.vel;
    player.vel += GRAVITY;

    let deflection =
        collide(player.bounds(), d.get_colliders_near(player.bounds().center()), Some(d.game_data.level.bounds()));
    player.pos += deflection.as_fvec2();

    if player.vel.y > 0.0 && deflection.y < 0 {
        player.vel.y = 0.0;
    }
}