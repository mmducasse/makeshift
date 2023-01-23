use macroquad::prelude::{is_key_pressed, is_key_down};
use xf::{time::timer::Timer, data::dir_h::DirH};

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, 
    entities::entity::Entity, 
};

use super::{
    player::Player, 
    state::State, 
    consts::*, 
    state_jump, 
    state_normal, state_util::check_collide_enemy
};

pub fn start(dir: DirH, player: &mut Player) {
    player.dir = dir;
    player.state = State::Dash;
    player.state_timer = Timer::new(DASH_TIME_S);
    player.d.vel.x = dir.unit().x as f32 * DASH_SPEED_X;
    //global::player_state::get_mut().last_checkpoint_pos = player.pos;
}

pub fn update(player: &mut Player, g: &mut GameData) {
    if !is_key_down(DASH_KEY) ||
       player.state_timer.is_done() {
        state_normal::start(player, g);
    }

    if is_key_pressed(JUMP_KEY) {
        state_jump::start(player);
    }

    player.d.pos += player.d.vel;
    player.d.vel += GRAVITY;

    let deflection =
        collide(player.bounds(), get_colliders_near(player.bounds().center(), &g), Some(g.level.bounds()));
    player.d.pos += deflection.as_fvec2();

    if player.d.vel.y > 0.0 && deflection.y < 0 {
        player.d.vel.y = 0.0;
    }

    check_collide_enemy(player, g);
}