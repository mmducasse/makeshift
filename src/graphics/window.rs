use macroquad::window::request_new_screen_size;
use xf::num::fvec2::FVec2;

use crate::consts::SCREEN_SIZE;

pub fn set_scale(scale: usize) {
    let desired_size = SCREEN_SIZE.as_fvec2() * FVec2::splat(scale as f32);
    request_new_screen_size(desired_size.x, desired_size.y);
}