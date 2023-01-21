use macroquad::prelude::{KeyCode, is_key_down};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::collide, 
    consts::GRAVITY,
};

use super::{
    player::Player, 
    state::State, 
    consts::*, 
    state_dash, 
    update_data::PlayerUpdateData, 
    state_normal
};

pub fn start(player: &mut Player) {
    player.pos.y -= 1.0;
    player.vel.y = JUMP_VEL_Y;
    player.state = State::Jump;
}

pub fn update(player: &mut Player, d: &mut PlayerUpdateData) {
    if let Some(dir) = get_dir_h_down() {
        player.dir = dir;
        let speed = player.vel.x.abs().max(RUN_SPEED_X);
        player.vel.x = dir.unit().x as f32 * speed;
    } else {
        player.vel.x = 0.0;
    }

    if !is_key_down(JUMP_KEY) {
        player.vel.y = player.vel.y.max(JUMP_RELEASE_VEL_Y);
    }

    player.pos += player.vel;
    player.vel += GRAVITY;

    player.vel.y = player.vel.y.min(MAX_FALL_VEL_Y);

    let deflection = collide(player.bounds(), d.get_colliders_near(player.bounds().center()), Some(d.game_data.level.bounds()));
    player.pos += deflection.as_fvec2();

    if player.vel.y > 0.0 && deflection.y < 0 {
        player.vel.y = 0.0;
        if is_key_down(DASH_KEY) {
            state_dash::start(player.dir, player);
        } else {
            state_normal::start(player, d);
        }
    }
    else if player.vel.y < JUMP_RELEASE_VEL_Y && deflection.y > 0 {
        player.vel.y = JUMP_RELEASE_VEL_Y;
    }

    if player.vel.y > WALLSLIDE_VEL_Y &&
       (player.vel.x * deflection.x as f32) < 0.0 {
        player.state = State::WallSlide
    }
}