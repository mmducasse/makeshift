use xf::num::ivec2::IVec2;

use crate::game::{game_data::GameData, draw_data::DrawData};

use super::{data::EntityData, type_::EntityType, entity::Entity};


pub struct Dummy {
    pub d: EntityData,
}

impl Dummy {
    pub fn new() -> Self {
        Self {
            d: EntityData::new(EntityType::None),
        }
    }
}

impl Entity for Dummy {
    fn data(&self) -> &EntityData { &self.d}
    fn data_mut(&mut self) -> &mut EntityData { &mut self.d}

    fn update(&mut self, g: &mut GameData) { }

    fn draw(&self, d: &mut DrawData, org: IVec2) { }
}