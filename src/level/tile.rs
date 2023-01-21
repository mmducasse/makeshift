use std::str::FromStr;

use xf::data::void::Void;


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub type_: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Self { type_: TileType::Empty }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Wall,
    Breakable,
}

impl TileType {
    pub fn is_impassable(self) -> bool {
        use TileType::*;

        matches!(self, Wall | Breakable)
    }
}

impl FromStr for TileType {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Wall" => TileType::Wall,
            "Breakable" => TileType::Breakable,
            _ => TileType::Empty,
        })
    }
}