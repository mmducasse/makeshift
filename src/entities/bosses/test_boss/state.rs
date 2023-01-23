use xf::num::ivec2::IVec2;

use crate::{
    game::game_data::GameData, 
    entities::bosses::test_boss::{state_normal, state_jump, state_hurt, state_fly, state_dash}
};

use super::{test_boss::TestBoss, anim::AnimKey};



#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Idle,
    RunTo(i32),
    DashTo(i32),
    Jump,
    FlyTo { target: IVec2, speed: f32},
    Float,
    Hurt,
}

impl State {
    pub fn to_anim_key(self, boss: &TestBoss) -> AnimKey {
        use State::*;

        let dir = boss.dir;

        match self {
            Idle => AnimKey::Idle(dir),
            RunTo(_) => AnimKey::Run(dir),
            DashTo(_) => AnimKey::Dash(dir),
            Jump | FlyTo { .. }  | Float => if boss.d.vel.y < 0.0 {
                AnimKey::JumpUp(dir)
            } else {
                AnimKey::JumpDown(dir)
            },
            Hurt => AnimKey::Hurt(dir),
        }
    }

    pub fn update(self, boss: &mut TestBoss, g: &mut GameData) {
        use State::*;

        match self {
            Idle | RunTo(_) => { state_normal::update(boss, g); },
            DashTo(x) => { state_dash::update(boss, x, g); },
            Jump => { state_jump::update(boss, g); },
            FlyTo { .. } | Float => { state_fly::update(boss, g); },
            Hurt => { state_hurt::update(boss, g); },
        }
    }
}