use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{
    io::controller::get_dir_h_down, 
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, entities::entity::Entity,
};

use super::{test_boss::TestBoss, state::State};

pub fn update(boss: &mut TestBoss, g: &GameData) {
    boss.state = if boss.d.vel.x == 0.0 {
        State::Idle
    } else {
        State::Run
    };


    boss.d.pos += boss.d.vel;
    boss.d.vel += GRAVITY;

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if boss.d.vel.y > 0.0 && deflection.y < 0 {
        boss.d.vel.y = 0.0;
    }

    if boss.d.vel.y > GRAVITY.y * 4.0 {
        boss.state = State::Jump;
    }
}