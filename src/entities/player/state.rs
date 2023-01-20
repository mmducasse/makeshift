use crate::{
    entities::player::{state_normal, state_jump, state_dash, state_wallslide}, game::game_data::GameData,
};

use super::{player::Player, anim::AnimKey, update_data::PlayerUpdateData};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Run,
    Jump,
    Dash,
    WallSlide,
}

impl State {
    pub fn to_anim_key(self, player: &Player) -> AnimKey {
        use State::*;

        let dir = player.dir;

        match self {
            Idle => AnimKey::Idle(dir),
            Run => AnimKey::Run(dir),
            Jump => if player.vel.y < 0.0 {
                    AnimKey::JumpUp(dir)
                } else {
                    AnimKey::JumpDown(dir)
                },
            Dash => AnimKey::Dash(dir),
            WallSlide => AnimKey::WallSlide(dir),
        }
    }

    pub fn update(self, player: &mut Player, d: &mut PlayerUpdateData) {
        use State::*;

        match self {
            Idle | Run => { state_normal::update(player, d); },
            Jump => { state_jump::update(player, d); },
            Dash => { state_dash::update(player, d); },
            WallSlide => { state_wallslide::update(player, d); },
        }
    }
}