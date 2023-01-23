
use xf::{num::{ivec2::IVec2, fvec2::FVec2}, data::dir_h::DirH};

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{test_boss::TestBoss, state::State, state_util::check_collide_enemy, consts::{RUN_SPEED_X, DASH_SPEED_X, FLY_SPEED}};

pub fn update(boss: &mut TestBoss, g: &mut GameData) {
    if let State::FlyTo(target) = boss.state {
        let delta = target.as_fvec2() - boss.bounds().center().as_fvec2();
        boss.dir = if delta.x < 0.0 { DirH::L } else { DirH::R };
        boss.d.vel = delta.normalize() * FLY_SPEED;
        boss.d.pos += boss.d.vel;
    } else {
        boss.d.vel = FVec2::ZERO;
    }

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    check_collide_enemy(boss, g);
}