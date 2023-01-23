
use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{test_boss::TestBoss, state::State, state_util::check_collide_enemy, consts::{RUN_SPEED_X, DASH_SPEED_X}};

pub fn update(boss: &mut TestBoss, g: &mut GameData) {
    

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