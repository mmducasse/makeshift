
use macroquad::prelude::is_key_pressed;
use xf::{data::dir_h::DirH, num::ivec2::{IVec2, i2}};

use crate::{game::game_data::GameData, consts::{P16, P8}, level::tile::TileType, entities::entity::Entity};

use super::{test_boss::TestBoss, consts::RUN_SPEED_X};


#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Running,
}

pub struct Ai {
    state: State,
}

const REVERSE_PROBE: IVec2 = i2(P8.x, -P16.y);
const JUMP_PROBE: IVec2 = i2(P8.x, 0);

impl Ai {
    pub fn new() -> Self {
        Self { state: State::Running }
    }

    pub fn update(boss: &mut TestBoss, g: &mut GameData) {
        match boss.ai.state {
            State::Idle => {
                boss.d.vel.x = 0.0;
            },
            State::Running => {
                let u = i2(boss.dir.unit().x, 1);
                boss.d.vel.x = u.x as f32 * RUN_SPEED_X;

                if test(boss, u * REVERSE_PROBE, g) {
                    boss.dir = boss.dir.opposite();
                } else if test(boss, u * JUMP_PROBE, g) {
                    boss.jump();
                }
            },
        }

        check_for_debug_inputs(boss);
    }
}

fn test(boss: &TestBoss, offset: IVec2, g: &GameData) -> bool {
    let pos = boss.bounds().center() + offset;
    let tile_type = g.level.tile_type_at(pos);
    tile_type != TileType::Empty
}

fn check_for_debug_inputs(boss: &mut TestBoss) {
    use macroquad::prelude::KeyCode;
    use State::*;
    const STATES: &[(KeyCode, State)] = &[
        (KeyCode::Key1, State::Idle),
        (KeyCode::Key2, State::Running),
    ];

    for &(key_code, state) in STATES {
        if is_key_pressed(key_code) {
            boss.ai.state = state;
        }
    }
}