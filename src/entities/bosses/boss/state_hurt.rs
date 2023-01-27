
use xf::time::countdown::Countdown;

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    game::game_data::GameData, 
    entities::entity::Entity,
};

use super::{state::State, consts::*, boss::Boss};


pub fn start(boss: &mut Boss, damage: i32, g: &mut GameData) {
    boss.state = State::Hurt;
    boss.grace_timer = Countdown::new(GRACE_TIME);
    g.boss_health -= damage;
}

pub fn update(boss: &mut Boss, g: &GameData) {
    boss.d.vel.x = 0.0;
    boss.d.vel.y = 0.0;
    boss.d.pos += boss.d.vel;

    let deflection = collide(boss.bounds(), get_colliders_near(boss.bounds().center(), &g), Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if boss.animator.is_done() {
        boss.state = State::Idle;
    }
}