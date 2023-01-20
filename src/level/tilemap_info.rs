use super::tileset_info::{TilesetInfo, TilesetId};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TilemapId {
    Test,
}

impl TilemapId {
    pub const fn info(self) -> TilemapInfo {
        use TilemapId::*;

        match self {
            Test => TEST_TILEMAP,
        }
    }
}

pub struct TilemapInfo {
    pub tilemap: &'static [u8],
    pub tileset_info: &'static TilesetInfo,
}

const TEST_TILEMAP: TilemapInfo = TilemapInfo {
    tilemap: include_bytes!("../../assets/Tilemaps/TestMap.tmj"),
    tileset_info: &TilesetId::Test.info(),
};