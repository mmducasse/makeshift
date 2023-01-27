
use macroquad::miniquad::start;

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{boss::Boss, state::State, state_util::check_collide_enemy, consts::{RUN_SPEED_X, DASH_SPEED_X, MARGIN}};

pub fn update(boss: &mut Boss, from_x: i32, to_x: i32, start_t: u64, g: &mut GameData) {
    let dt = g.frame_num() - start_t;
    let dx = i32::signum(to_x - from_x) as f32 * DASH_SPEED_X;
    boss.d.pos.x = from_x as f32 + dx;

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    check_collide_enemy(boss, g);
}