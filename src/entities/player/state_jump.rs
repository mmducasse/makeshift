use macroquad::prelude::{is_key_down};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, game::game_data::GameData, entities::entity::Entity,
};

use super::{
    player::Player, 
    state::State, 
    consts::*, 
    state_dash, 
    state_normal, state_util::check_collide_enemy
};

pub fn start(player: &mut Player) {
    player.d.pos.y -= 1.0;
    player.d.vel.y = JUMP_VEL_Y;
    player.state = State::Jump;
}

pub fn update(player: &mut Player, g: &mut GameData) {
    if let Some(dir) = get_dir_h_down() {
        player.dir = dir;
        let speed = player.d.vel.x.abs().max(RUN_SPEED_X);
        player.d.vel.x = dir.unit().x as f32 * speed;
    } else {
        player.d.vel.x = 0.0;
    }

    if !is_key_down(JUMP_KEY) {
        player.d.vel.y = player.d.vel.y.max(JUMP_RELEASE_VEL_Y);
    }

    player.d.pos += player.d.vel;
    player.d.vel += GRAVITY;

    player.d.vel.y = player.d.vel.y.min(MAX_FALL_VEL_Y);

    let deflection = collide(player.bounds(), get_colliders_near(player.bounds().center(), &g), Some(g.level.bounds()));
    player.d.pos += deflection.as_fvec2();

    if player.d.vel.y > 0.0 && deflection.y < 0 {
        player.d.vel.y = 0.0;
        if is_key_down(DASH_KEY) {
            state_dash::start(player.dir, player);
        } else {
            state_normal::start(player, g);
        }
    }
    else if player.d.vel.y < JUMP_RELEASE_VEL_Y && deflection.y > 0 {
        player.d.vel.y = JUMP_RELEASE_VEL_Y;
    }

    if player.d.vel.y > WALLSLIDE_VEL_Y &&
       (player.d.vel.x * deflection.x as f32) < 0.0 {
        player.state = State::WallSlide
    }

    check_collide_enemy(player, g);
}