use xf::num::ivec2::IVec2;

use crate::{
    game::game_data::GameData, 
    entities::bosses::boss::{state_idle, state_hurt, state_go}, 
    consts::GRAVITY
};

use super::{boss::Boss, anim::AnimKey};

#[derive(Clone, Copy, PartialEq)]
pub enum GoType {
    Run,
    Dash,
    Fly(f32),
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Idle,
    Float,
    GoTo { 
        type_: GoType, 
        from: IVec2, 
        to_x: i32, 
        to_y: Option<i32>,
        start_t: u64 
    },
    Hurt,
}

impl State {
    pub fn to_anim_key(self, boss: &Boss) -> AnimKey {
        use State::*;

        let dir = boss.dir;

        match self {
            Idle => if boss.d.vel.y > GRAVITY.y * 4.0 {
                AnimKey::JumpDown(dir)
            } else {
                 AnimKey::Idle(dir)
            },
            Float => AnimKey::JumpDown(dir),
            GoTo { type_, .. } => match type_ {
                GoType::Run => AnimKey::Run(dir),
                GoType::Dash => AnimKey::Dash(dir),
                GoType::Fly(_) => if boss.d.vel.y < 0.0 {
                    AnimKey::JumpUp(dir)
                } else {
                    AnimKey::JumpDown(dir)
                },
            },
            Hurt => AnimKey::Hurt(dir),
        }
    }

    pub fn update(self, boss: &mut Boss, g: &mut GameData) {
        use State::*;

        match self {
            Idle => { state_idle::update(boss, true, g); },
            Float => { state_idle::update(boss, false, g); },
            Hurt => { state_hurt::update(boss, g); },
            GoTo { type_, from, to_x, to_y, start_t } =>
                state_go::update(boss, type_, from, to_x, to_y, start_t, g),
        }
    }
}