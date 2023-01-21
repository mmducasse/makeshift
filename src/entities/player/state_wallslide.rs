use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{systems::collision::{collide, get_colliders_near}, game::game_data::GameData, entities::entity::Entity};

use super::{
    player::Player,
    consts::*, 
    state_jump,
    state_normal
};


pub fn update(player: &mut Player, g: &mut GameData) {
    player.d.vel.x = player.dir.unit().x as f32 * WALLSLIDE_VEL_Y;
    player.d.vel.y = WALLSLIDE_VEL_Y;
    player.d.pos += player.d.vel;

    let deflection = collide(player.bounds(), get_colliders_near(player.bounds().center(), &g), Some(g.level.bounds()));
    player.d.pos += deflection.as_fvec2();

    if is_key_pressed(JUMP_KEY) {
        state_jump::start(player);
        return;
    }
    else if player.d.vel.y > 0.0 && deflection.y < 0 {
        player.d.vel.y = 0.0;
        state_normal::start(player, g);
    }
    else if deflection.x == 0 {
        state_normal::start(player, g);
    }
}