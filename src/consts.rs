
use xf::num::{ivec2::{i2, IVec2}, fvec2::{FVec2, f2}};

pub const GRAVITY: FVec2 = f2(0.0, 0.2);

pub const P8: IVec2 = i2(8, 8);
pub const P16: IVec2 = i2(16, 16);

pub const SCREEN_P16_SIZE: IVec2 =  i2(14, 12); //
pub const SCREEN_SIZE: IVec2 = IVec2::mul(SCREEN_P16_SIZE, P16);