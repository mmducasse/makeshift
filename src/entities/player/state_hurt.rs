use macroquad::prelude::{is_key_pressed, KeyCode};
use xf::{num::ivec2::i2, time::{timer::Timer, countdown::Countdown}};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, 
    entities::{entity::Entity, spawn::spawn_entity, bullets::bullet::Bullet},
};

use super::{player::Player, state::State, consts::*, state_jump, state_dash};


pub fn start(player: &mut Player, damage: i32, g: &GameData) {
    player.state = State::Hurt;
    player.grace_timer = Countdown::new(GRACE_TIME);
}

pub fn update(player: &mut Player, g: &GameData) {
    player.d.vel.x = player.dir.unit().x as f32 * -0.3;
    player.d.vel.y = 0.0;
    player.d.pos += player.d.vel;

    let deflection = collide(player.bounds(), get_colliders_near(player.bounds().center(), &g), Some(g.level.bounds()));
    player.d.pos += deflection.as_fvec2();

    if player.animator.is_done() {
        player.state = State::Idle;
    }
}