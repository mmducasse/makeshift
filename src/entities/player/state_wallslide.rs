use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::systems::collision::collide;

use super::{
    player::Player,
    consts::*, 
    state_jump, 
    update_data::PlayerUpdateData, 
    state_normal
};


pub fn update(player: &mut Player, d: &mut PlayerUpdateData) {
    player.vel.x = player.dir.unit().x as f32 * WALLSLIDE_VEL_Y;
    player.vel.y = WALLSLIDE_VEL_Y;
    player.pos += player.vel;

    let deflection = collide(player.bounds(), d.get_colliders_near(player.bounds().center()), Some(d.game_data.level.bounds()));
    player.pos += deflection.as_fvec2();

    if is_key_pressed(JUMP_KEY) {
        state_jump::start(player);
        return;
    }
    else if player.vel.y > 0.0 && deflection.y < 0 {
        player.vel.y = 0.0;
        state_normal::start(player, d);
    }
    else if deflection.x == 0 {
        state_normal::start(player, d);
    }
}