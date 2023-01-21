use xf::num::{ivec2::IVec2, irect::IRect};

use crate::{game::game_data::GameData, entities::entity::Entity};


pub struct Camera {
    pos: IVec2,
    size: IVec2,
}

impl Camera {
    pub fn new(pos: IVec2, size: IVec2) -> Self {
        Self { pos, size }
    }

    //pub fn update(subject_a_bounds: IRect, subject_b_bounds: IRect) {  
    pub fn update(&mut self, g: &GameData) {
        let Some(player) = &g.player else { return };
        let bounds_a = player.bounds();
        let scene_bounds = g.level.bounds();

        let desired_center = bounds_a.center();
        let desired_view = IRect::centered_at(desired_center, self.size);

        self.pos = desired_view.keep_inside(scene_bounds).pos;
    }

    pub fn pos(&self) -> IVec2 { self.pos }
}