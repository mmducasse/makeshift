
use xf::num::{ivec2::IVec2, irect::{IRect, rect, ir}};

use crate::{
    level::level::Level, 
    consts::P16, game::game_data::GameData
};



// pub struct PlayerUpdateData<'a> {
//     pub game_data: &'a mut GameData,
// }

// impl<'a> PlayerUpdateData<'a> {
//     /// Returns colliders for all tiles.
//     pub fn get_colliders_near(&self, center: IVec2) -> Vec<IRect> {
//         const AREA: IRect = rect(-1, -1, 3, 3);
//         let mut vec = vec![];

//         let pos_p16 = center / P16;

//         // Get bounds of colliding tiles.
//         let tilemap = &self.game_data.level.tilemap;

//         for offset in AREA.iter() {
//             let tile_p16_pos = pos_p16 + offset;
//             if let Some(tile) = tilemap.get(tile_p16_pos) {
//                 if tile.type_.is_impassable() {
//                     let tile_bounds = ir(tile_p16_pos * P16, P16);
    
//                     vec.push(tile_bounds);
//                 }
//             }
//         }

//         // // Get bounds of colliding entities.
//         // for e in self.level.entities.all() {
//         //     if e.collides() {
//         //         vec.push(e.bounds());
//         //     }
//         // }


//         vec
//     }
// }