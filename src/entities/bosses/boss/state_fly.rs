
use xf::{num::{ivec2::IVec2, fvec2::FVec2}, data::dir_h::DirH};

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{boss::Boss, state::State, state_util::check_collide_enemy, consts::{RUN_SPEED_X, DASH_SPEED_X, FLY_SPEED_SLOW}};

pub fn update(boss: &mut Boss, g: &mut GameData) {
    if let State::FlyTo { target, speed } = boss.state {
        let delta = target.as_fvec2() - boss.d.pos;
        boss.dir = if delta.x < 0.0 { DirH::L } else { DirH::R };

        if delta.magnitude() < speed {
            boss.d.pos = target.as_fvec2();
        } else {
            boss.d.vel = delta.normalize() * speed;
            boss.d.pos += boss.d.vel;
        }
    } else {
        boss.d.vel = FVec2::ZERO;
    }

    // let colliders = get_colliders_near(boss.bounds().center(), g);
    // let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    // boss.d.pos += deflection.as_fvec2();

    check_collide_enemy(boss, g);
}