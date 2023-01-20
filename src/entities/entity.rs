use xf::num::{ivec2::IVec2, irect::IRect};

use crate::game::{game_data::GameData, draw_data::DrawData};


static mut NEXT_ID: usize = 0;

pub fn next_entity_id() -> usize {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

pub trait Entity {
    fn id(&self) -> usize;
    fn bounds(&self) -> IRect;
    fn collides(&self) -> bool { false }
    fn update(&mut self, d: &mut GameData);
    fn draw(&self, d: &DrawData);
}