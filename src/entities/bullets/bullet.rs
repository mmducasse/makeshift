use xf::{num::{ivec2::IVec2, irect::{rect, IRect}, fvec2::FVec2}, gl::anim::Animator};

use crate::{entities::{data::EntityData, type_::EntityType}, consts::P16};

use super::{type_::BulletType, anim::animator};



pub struct Bullet {
    pub d: EntityData,
    pub(super) animator: Animator<BulletType>,
}

const COLLIDER: IRect = rect(4, 4, 8, 8);

impl Bullet {
    pub fn new(friendly: bool, center: IVec2, vel: FVec2) -> Self {
        let pos = center - (P16 / 2);
        let type_ = if friendly {
            EntityType::PlayerWeapon(1)
        } else {
            EntityType::EnemyWeapon(1)
        };

        Self {
            d: EntityData::new(type_)
                .at(pos)
                .with_vel(vel)
                .with_collider(COLLIDER),
            animator: animator(),
        }
    }
}