
use xf::data::dir_h::DirH;

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{test_boss::TestBoss, state::State, state_util::check_collide_enemy, consts::{RUN_SPEED_X, DASH_SPEED_X}};

pub fn update(boss: &mut TestBoss, g: &mut GameData) {
    if let State::RunTo(target_x) = boss.state {
        if f32::abs(boss.d.pos.x - target_x as f32) < RUN_SPEED_X {
            boss.d.pos.x = target_x as f32;
            boss.d.vel.x = 0.0;
        } else {
            boss.dir = DirH::from_x(target_x - boss.d.pos.x as i32);
            boss.d.vel.x = boss.dir.unit().x as f32 * RUN_SPEED_X;
        }
    } else {
        boss.d.vel.x = 0.0;
    }

    boss.d.vel += GRAVITY;
    boss.d.pos += boss.d.vel;

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if boss.d.vel.y > 0.0 && deflection.y < 0 {
        boss.d.vel.y = 0.0;
    }

    if boss.d.vel.y > GRAVITY.y * 4.0 {
        boss.state = State::Jump;
    }

    check_collide_enemy(boss, g);
}