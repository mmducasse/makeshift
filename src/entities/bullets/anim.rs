use xf::{gl::anim::{Animator, Animation}, num::ivec2::i2};

use crate::{entities::bullets::type_::BulletType, consts::P16, row_l};


pub fn animator() -> Animator<BulletType> {
    Animator::new(
        BulletType::Red, 
        P16, 
        map_fn
    )
}

fn map_fn(key: BulletType) -> &'static dyn Animation {
    use BulletType::*;
    const DUR: u32 = 4;

    match key {
        Red => row_l!(2, 1, i2(0, 0)),
    }
}