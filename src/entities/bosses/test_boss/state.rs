use xf::num::ivec2::IVec2;

use crate::{
    game::game_data::GameData, 
    entities::bosses::test_boss::{state_normal, state_jump, state_hurt, state_fly}
};

use super::{test_boss::TestBoss, anim::AnimKey};



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Run,
    Dash,
    Jump,
    FlyTo(IVec2),
    Float,
    Hurt,
}

impl State {
    pub fn to_anim_key(self, boss: &TestBoss) -> AnimKey {
        use State::*;

        let dir = boss.dir;

        match self {
            Idle => AnimKey::Idle(dir),
            Run => AnimKey::Run(dir),
            Dash => AnimKey::Dash(dir),
            Jump | FlyTo(_) | Float => if boss.d.vel.y < 0.0 {
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
            Idle | Run | Dash => { state_normal::update(boss, g); },
            Jump => { state_jump::update(boss, g); },
            FlyTo(_) | Float => { state_fly::update(boss, g); },
            Hurt => { state_hurt::update(boss, g); },
        }
    }
}