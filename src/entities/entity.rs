use xf::num::{ivec2::IVec2, irect::{IRect, ir}};

use crate::game::{game_data::GameData, draw_data::DrawData};

use super::data::EntityData;


pub trait Entity {
    fn data(&self) -> &EntityData;
    fn data_mut(&mut self) -> &mut EntityData;

    fn update(&mut self, g: &mut GameData);
    fn draw(&self, d: &mut DrawData, org: IVec2);

    fn bounds(&self) -> IRect {
        self.data().collider.offset_by(
            self.data().pos.as_ivec2()
        )
    }
}