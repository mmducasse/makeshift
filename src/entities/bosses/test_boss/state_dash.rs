
use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{test_boss::TestBoss, state::State, state_util::check_collide_enemy, consts::{RUN_SPEED_X, DASH_SPEED_X, MARGIN}};

pub fn update(boss: &mut TestBoss, target_x: i32, g: &mut GameData) {
    if f32::abs(target_x as f32 - boss.d.pos.x) <= DASH_SPEED_X + MARGIN {
        boss.d.pos.x = target_x as f32;
        boss.d.vel.x = 0.0;
    } else {
        boss.d.vel.x = boss.dir.unit().x as f32 * DASH_SPEED_X;
        boss.d.vel.y = 0.0;
        boss.d.pos += boss.d.vel;
    }

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    check_collide_enemy(boss, g);
}