use xf::{gl::anim::{Animator, Animation}, data::dir_h::DirH};

use crate::{consts::P16, row_2_l, row_2};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimKey {
    Idle(DirH),
    Run(DirH),
    JumpUp(DirH),
    JumpDown(DirH),
    Dash(DirH),
    WallSlide(DirH),
    Hurt(DirH),
}

pub fn animator() -> Animator<AnimKey> {
    Animator::new(
        AnimKey::Idle(DirH::R), 
        P16, 
        map_fn
    )
}

fn map_fn(key: AnimKey) -> &'static dyn Animation {

    use AnimKey::*;
    const DUR: u32 = 4;

    match key {
        // Normal
        Idle(dir) => row_2_l!(dir, 1, DUR, i2(0, 0)),
        Run(dir) => row_2_l!(dir, 4, DUR, i2(0, 2)),
        JumpUp(dir) => row_2_l!(dir, 1, DUR, i2(2, 2)),
        JumpDown(dir) => row_2_l!(dir, 1, DUR, i2(0, 2)),
        Dash(dir) => row_2_l!(dir, 1, DUR, i2(0, 4)),
        WallSlide(dir) => row_2_l!(dir, 1, DUR, i2(0, 6)),
        Hurt(dir) => row_2!(dir, 1, 16, i2(0, 14)),
    }
}