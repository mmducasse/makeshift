use macroquad::prelude::{KeyCode, is_key_down};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, game::game_data::GameData, entities::entity::Entity,
};

use super::{
    state::State, 
    consts::{JUMP_RELEASE_VEL_Y, JUMP_VEL_Y, WALLSLIDE_VEL_Y, RUN_SPEED_X, MAX_FALL_VEL_Y},
    state_normal, test_boss::TestBoss
};

pub fn update(boss: &mut TestBoss, g: &mut GameData) {
    boss.d.pos += boss.d.vel;
    boss.d.vel += GRAVITY;

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if deflection.y < 0 {
        boss.d.vel.y = 0.0;
        boss.state = State::Idle;
    }

    if boss.d.vel.y > GRAVITY.y * 4.0 {
        boss.state = State::Jump;
    }
}