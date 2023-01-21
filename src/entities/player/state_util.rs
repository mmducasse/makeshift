use crate::{entities::{type_::EntityType, entity::Entity}, game::game_data::GameData};

use super::{state_hurt, player::Player};



pub fn check_collide_enemy(player: &mut Player, g: &mut GameData) {
    if !player.grace_timer.is_done() { return; }

    for e in g.entities.iter() {
        match e.data().type_ {
            EntityType::Enemy(dmg) | EntityType::EnemyWeapon(dmg) => {
                if player.bounds().intersection(e.bounds()).is_some() {
                    state_hurt::start(player, dmg, g);
                    return;
                }
            },
            _ => {},
        }
    }
}