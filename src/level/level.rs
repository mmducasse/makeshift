use xf::{map::tilemap::Tilemap, num::{ivec2::{IVec2, i2}, irect::{IRect, ir}}, gl::bitmap::Bitmap};

use crate::{consts::P16, game::{draw_data::DrawData}, other::background::Background};

use super::tile::{Tile, TileType};



pub struct Level {
    pub tilemap: Tilemap<Tile>,
    pub background: Background,
}

impl Level {
    pub fn new(tilemap: Tilemap<Tile>) -> Self {
        Self {
            tilemap,
            background: Background::new(),
        }
    }

    pub fn update(&mut self) {
        self.background.update();
    }

    pub fn draw(&self, view: IRect, d: &mut DrawData) {
        self.background.draw(d, view.pos);

        let view_p16 = ir(view.pos / P16, view.size / P16).expand(1); // todo: expand by 1

        for tile_pos_p16 in view_p16.iter() {
            if let Some(Some(src)) = self.tilemap.tile_srcs.get(tile_pos_p16) {
                let tile = self.tilemap.tileset.tiles.get(*src).unwrap();
                //let frames = tile.frames.unwrap_or(1);

                let src_offset_x = 0; //(frame_num() / TILE_ANIMATION_RATE) % frames;
                let src = *src + i2(src_offset_x as i32, 0);

                let src = ir(src * P16, P16);
                let dst_pt = tile_pos_p16 * P16;
    
                d.buffer().draw_texture(&self.tilemap.tileset.texture, src, dst_pt - view.pos);
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

        if !self.bounds().contains(pos) { return TileType::OutOfBounds }

        self.tilemap.get(pos_p16)
                    .map(|t| t.type_)
                    .unwrap_or(TileType::Empty)
    }
}