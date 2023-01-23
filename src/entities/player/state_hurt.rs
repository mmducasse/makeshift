
use xf::{time::{countdown::Countdown}};

use crate::{
    systems::collision::{collide, get_colliders_near},
    game::game_data::GameData, 
    entities::{entity::Entity},
};

use super::{player::Player, state::State, consts::*};


pub fn start(player: &mut Player, damage: i32, g: &mut GameData) {
    player.state = State::Hurt;
    player.grace_timer = Countdown::new(GRACE_TIME);
    g.player_health -= damage;
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