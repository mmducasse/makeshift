use macroquad::prelude::KeyCode;


pub const RUN_SPEED_X: f32 = 2.0;
pub const DASH_SPEED_X: f32 = 4.0;
pub const JUMP_VEL_Y: f32 = -5.0;
pub const JUMP_RELEASE_VEL_Y: f32 = -2.0;
pub const MAX_FALL_VEL_Y: f32 = 5.0;
pub const WALLSLIDE_SPEED_X: f32 = 0.5;
pub const WALLSLIDE_VEL_Y: f32 = 1.0;

pub const BULLET_SPEED: f32 = 4.0;

pub const DASH_TIME_S: u32 = 10;
pub const GRACE_TIME: u32 = 60;

pub const JUMP_KEY: KeyCode = KeyCode::Space;
pub const DASH_KEY: KeyCode = KeyCode::Down;