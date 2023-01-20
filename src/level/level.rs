use xf::{map::tilemap::Tilemap, num::{ivec2::{IVec2, i2}, irect::{IRect, ir}}, gl::bitmap::Bitmap};

use crate::{consts::P16, game::draw_data::DrawData};

use super::tile::{Tile, TileType};



pub struct Level {
    pub tilemap: Tilemap<Tile>,
}

impl Level {
    pub fn draw(&self, view: IRect, d: &mut DrawData) {

        let view_p16 = ir(view.pos / P16, view.size / P16).expand(1);

        for tile_pos_p16 in view_p16.iter() {
            if let Some(&src) = self.tilemap.tile_srcs.get(tile_pos_p16) {
                if let Some(src) = src {
                    let tile = self.tilemap.tileset.tiles.get(src).unwrap();
                    //let frames = tile.frames.unwrap_or(1);

                    let src_offset_x = 0; //(frame_num() / TILE_ANIMATION_RATE) % frames;
                    let src = src + i2(src_offset_x as i32, 0);

                    let src = ir(src * P16, P16);
                    let dst_pt = tile_pos_p16 * P16;
        
                    d.buffer().draw_texture(&self.tilemap.tileset.texture, src, dst_pt - view.pos);
                }
            }
        }
    }

    pub fn p16_size(&self) -> IVec2 {
        self.tilemap.size()
    }

    pub fn bounds(&self) -> IRect {
        IRect::of_size(self.p16_size() * P16)
    }

    pub fn tile_type_at(&self, pos: IVec2) -> TileType {
        let pos_p16 = pos / P16;
        if let Some(tile) = self.tilemap.get(pos_p16) {
            tile.type_
        } else {
            TileType::Empty
        }
    }
}