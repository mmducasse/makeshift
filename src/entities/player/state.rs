use crate::{
    entities::player::{state_normal, state_jump, state_dash, state_wallslide, state_hurt}, game::game_data::GameData,
};

use super::{player::Player, anim::AnimKey};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Run,
    Jump,
    Dash,
    WallSlide,
    Hurt,
    //Dead,
}

impl State {
    pub fn to_anim_key(self, player: &Player) -> AnimKey {
        use State::*;

        let dir = player.dir;

        match self {
            Idle => AnimKey::Idle(dir),
            Run => AnimKey::Run(dir),
            Jump => if player.d.vel.y < 0.0 {
                    AnimKey::JumpUp(dir)
                } else {
                    AnimKey::JumpDown(dir)
                },
            Dash => AnimKey::Dash(dir),
            WallSlide => AnimKey::WallSlide(dir),
            Hurt => AnimKey::Hurt(dir),
        }
    }

    pub fn update(self, player: &mut Player, g: &mut GameData) {
        use State::*;

        match self {
            Idle | Run => { state_normal::update(player, g); },
            Jump => { state_jump::update(player, g); },
            Dash => { state_dash::update(player, g); },
            WallSlide => { state_wallslide::update(player, g); },
            Hurt => { state_hurt::update(player, g); },
        }
    }
}