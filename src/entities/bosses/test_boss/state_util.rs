use crate::{entities::{type_::EntityType, entity::Entity}, game::game_data::GameData};

use super::{state_hurt, test_boss::TestBoss};



pub fn check_collide_enemy(boss: &mut TestBoss, g: &mut GameData) {
    if !boss.grace_timer.is_done() { return; }

    for e in g.entities.iter() {
        match e.data().type_ {
            EntityType::PlayerWeapon(dmg) => {
                if boss.bounds().intersection(e.bounds()).is_some() {
                    state_hurt::start(boss, dmg, g);
                    return;
                }
            },
            _ => {},
        }
    }
}