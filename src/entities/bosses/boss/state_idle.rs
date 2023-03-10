
use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{boss::Boss, state_util::check_collide_enemy};

pub fn update(boss: &mut Boss, has_gravity: bool, g: &mut GameData) {
    boss.d.vel.x = 0.0;

    if has_gravity {
        boss.d.vel += GRAVITY;
        boss.d.pos += boss.d.vel;
    }

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if boss.d.vel.y > 0.0 && deflection.y < 0 {
        boss.d.vel.y = 0.0;
    }

    check_collide_enemy(boss, g);
}