use macroquad::prelude::{is_key_pressed, KeyCode};
use xf::{num::ivec2::i2, time::{timer::Timer, countdown::Countdown}};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, 
    entities::{entity::Entity, spawn::spawn_entity, bullets::bullet::Bullet},
};

use super::{state::State, consts::*, test_boss::TestBoss};


pub fn start(boss: &mut TestBoss, damage: i32, g: &GameData) {
    boss.state = State::Hurt;
    boss.grace_timer = Countdown::new(GRACE_TIME);
}

pub fn update(boss: &mut TestBoss, g: &GameData) {
    boss.d.vel.x = 0.0;
    boss.d.vel.y = 0.0;
    boss.d.pos += boss.d.vel;

    let deflection = collide(boss.bounds(), get_colliders_near(boss.bounds().center(), &g), Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if boss.animator.is_done() {
        boss.state = State::Idle;
    }
}