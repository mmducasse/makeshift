use crate::game::game_data::GameData;

use super::entity::Entity;



pub fn spawn_entity<E>(entity: E, g: &mut GameData)
where E: Entity + 'static
{
    g.entities.add(Box::new(entity));
}