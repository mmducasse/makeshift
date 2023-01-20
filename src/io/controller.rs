use macroquad::prelude::{is_key_down, KeyCode};
use xf::{data::{dir4::Dir4, dir_h::DirH}, num::ivec2::IVec2};

const KEY_DIRS: [(KeyCode, Dir4); 4] = [
    (KeyCode::Up, Dir4::N),
    (KeyCode::Right, Dir4::E),
    (KeyCode::Down, Dir4::S),
    (KeyCode::Left, Dir4::W),
];


pub fn get_dir_down() -> IVec2 {
    let mut total_dir = IVec2::ZERO;

    for (code, dir) in KEY_DIRS {
        if is_key_down(code) {
            total_dir += dir.unit();
        }
    }

    total_dir
}

pub fn get_dir_h_down() -> Option<DirH> {
    if is_key_down(KeyCode::Left) {
        Some(DirH::L)
    } else if is_key_down(KeyCode::Right) {
        Some(DirH::R)
    } else {
        None
    }
}