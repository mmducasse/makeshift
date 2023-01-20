
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TilesetId {
    Test
}

impl TilesetId {
    pub const fn info(self) -> TilesetInfo {
        use TilesetId::*;

        match self {
            Test => TEST_TILESET,
        }
    }
}

pub struct TilesetInfo {
    pub tileset: &'static [u8],
    pub image: &'static [u8],
}

const TEST_TILESET: TilesetInfo = TilesetInfo {
    tileset: include_bytes!("../../assets/Tilesets/TestTileset.tsj"),
    image: include_bytes!("../../assets/TestTileset.png"),
};