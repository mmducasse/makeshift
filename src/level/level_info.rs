use super::tilemap_info::{TilemapInfo, TilemapId};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LevelId {
    Test,
}

impl LevelId {
    pub const fn info(self) -> LevelInfo {
        use LevelId::*;

        match self {
            Test => TEST_LEVEL,
        }
    }
}

pub struct LevelInfo {
    pub tilemap_info: TilemapInfo,
}

const TEST_LEVEL: LevelInfo = LevelInfo {
    tilemap_info: TilemapId::Test.info(),
};