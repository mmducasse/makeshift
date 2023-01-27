use core::time;
use std::f32::consts::PI;

use xf::{data::{spin::Spin, dir_h::DirH}, num::{ivec2::{IVec2, i2}, fvec2::f2}, time::{countdown::Countdown}};

use crate::{
    game::game_data::GameData, 
    entities::{entity::Entity, bosses::test_boss::{state::{self, GoType}, consts::{FLY_SPEED_SLOW, FLY_SPEED_FAST, RUN_SPEED_X, DASH_SPEED_X}}, spawn::spawn_entity, bullets::bullet::Bullet}, 
    consts::{P16, P8}
};

use super::{test_boss::TestBoss, consts::RESET_TIME};

const T_IDLE: u32 = 30;
const TIMEOUT: u32 = 300;

const X_0: i32 = 1 * P16.x;
const X_1: i32 = 2 * P16.x;
const X_2: i32 = (6 * P16.x) + P8.x;
const X_3: i32 = 11 * P16.x;
const X_4: i32 = 12 * P16.x;

const Y_TOP: i32 = 1 * P16.y;
const Y_MID: i32 = 5 * P16.y;
const Y_BOTTOM: i32 = 9 * P16.y;

const CENTER_POINT: IVec2 = i2(X_2, Y_MID);

#[derive(Clone, Copy, PartialEq, Debug)]
enum AiState {
    Idle(u32),
    Face(DirH),
    RunToX(i32),
    DashToX(i32),
    FlyToXY { target: IVec2, speed: f32},
    Float(u32),
    Shoot(ShotType),
    Whirl(Spin, u32),
    FloorCeil(u32),
}

impl AiState {
    pub fn time(self, pos: IVec2) -> u32 {
        use AiState::*;

        match self {
            Idle(t) | Whirl(_, t) | FloorCeil(t) | Float(t) => t,
            RunToX(x) => (i32::abs(pos.x - x) as f32 / RUN_SPEED_X) as u32,
            DashToX(x) => (i32::abs(pos.x - x) as f32 / DASH_SPEED_X) as u32,
            FlyToXY { target, speed } => ((target - pos).as_fvec2().magnitude() / speed) as u32,
            _ => 0,
        }
    }
}

pub struct Ai {
    state: AiState,
    state_timer: Countdown,
    queue: Vec<AiState>,
    can_reset: bool,
}

impl Ai {
    pub fn new() -> Self {
        Self {
            state: AiState::Idle(0),
            state_timer: Countdown::new(0),
            queue: vec![AiState::Idle(15)],
            can_reset: true,
        }
    }

    pub fn update(boss: &mut TestBoss, g: &mut GameData) {
        use AiState::*;

        if !boss.grace_timer.is_done() {
            if boss.ai.can_reset {
                reset_ai(boss);
                return;
            }
        } else {
            boss.ai.can_reset = true;
        }

        boss.ai.state_timer.decrement();
        
        if boss.ai.state_timer.is_done() {
            if boss.ai.queue.is_empty() {
                add_states(&mut boss.ai.queue, g.frame_num(), g.boss_health.value);
            }
            
            let state = boss.ai.queue.remove(0);
            boss.ai.state = state;
            boss.ai.state_timer = Countdown::new(boss.ai.state.time(boss.d.pos.as_ivec2()));

            println!("New state = {:?}", boss.ai.state);

            if let Shoot(type_) = boss.ai.state {
                shoot(type_, boss, g);
            }
        }

        let pos = boss.d.pos.as_ivec2();
        let t = g.frame_num();

        match boss.ai.state {
            Idle(_) => {
                boss.state = state::State::Idle;
            },
            Face(dir) => {
                boss.dir = dir;
            },
            RunToX(x) => {
                boss.state = state::State::GoTo { 
                    type_: GoType::Run, 
                    from: pos, to_x: x, to_y: None, 
                    start_t: t,
                }
            },
            DashToX(x) => {
                boss.state = state::State::GoTo { 
                    type_: GoType::Dash, 
                    from: pos, to_x: x, to_y: None, 
                    start_t: t,
                }
            },
            FlyToXY { target, speed } => {
                boss.state = state::State::GoTo { 
                    type_: GoType::Fly(speed), 
                    from: pos, to_x: target.x, to_y: Some(target.y), 
                    start_t: t,
                }
            },
            Float(_) => {
                boss.state = state::State::Float;
            },
            Shoot(type_) => { }
            _ => { todo!() }
        }
    }
}

fn reset_ai(boss: &mut TestBoss) {
    boss.ai.state = AiState::Idle(RESET_TIME);
    boss.ai.state_timer = Countdown::new(RESET_TIME);
    boss.ai.queue.clear();
    boss.ai.can_reset = false;
}


fn add_states(queue: &mut Vec<AiState>, seed: u64, health: i32) {
    use AiState::*;
    use ShotType::*;

    const T_WAVE: u32 = 10;

    let selection = if health > 12 {
        seed % 1
    } else if health > 10 {
        seed % 2
    } else {
        (seed % 6) + 1
    };

    let mut next = match selection {
        // Run to each side and shoot.
        0 => vec![
            RunToX(X_0), 
            RunToX(X_1), 
            Shoot(Single),
            Idle(20),
            RunToX(X_4),
            RunToX(X_3), 
            Shoot(Single),
            Idle(20),
        ],

        // Run and dash around.
        1 => vec![
            DashToX(X_1), 
            RunToX(X_0), 
            RunToX(X_1), 
            DashToX(X_3), 
            RunToX(X_4), 
            RunToX(X_3), 
        ],

        // Dash to each side and shoot.
        2 => vec![
            DashToX(X_0), 
            Face(DirH::R), 
            Shoot(Single),
            Idle(20),
            DashToX(X_4),
            Face(DirH::L),  
            Shoot(Single),
            Idle(20),
        ],

        // Dash around edges of arena.
        3 => vec![
            DashToX(X_0), 
            FlyToXY { target: i2(X_0, Y_TOP), speed: FLY_SPEED_FAST }, 
            DashToX(X_4), 
            FlyToXY { target: i2(X_4, Y_BOTTOM), speed: FLY_SPEED_FAST },
        ],

        // Shoot walls
        4 => vec![
            DashToX(X_3),
            Face(DirH::L),
            Idle(30),
            Shoot(Wall),
            Idle(30),
            Shoot(Wall),
            Idle(30),
            Shoot(Wall),
            Idle(30),
        ],

        // Float to middle and shoot bursts.
        5 => vec![
            DashToX(X_1), 
            DashToX(X_3), 
            DashToX(X_1), 
            DashToX(X_3), 
            DashToX(X_2), 
            FlyToXY { target: CENTER_POINT, speed: FLY_SPEED_SLOW }, 
            Shoot(PlusBurst { angle: 0.0 }), 
            Float(30), 
            Shoot(PlusBurst { angle: 0.125 }), 
            Float(30), 
            Shoot(PlusBurst { angle: 0.0 }), 
            Float(30), 
            Idle(60)
        ],

        // Cascade of bullets from ceiling.
        _ => vec![
            DashToX(X_0), 
            FlyToXY { target: i2(X_0, Y_TOP), speed: FLY_SPEED_FAST }, 
            DashToX(X_1), 
            Face(DirH::R),
            Idle(20),
            Shoot(Wall),
            Shoot(Down(3)),
            Idle(T_WAVE),
            Shoot(Down(4)),
            Idle(T_WAVE),
            Shoot(Down(5)),
            Idle(T_WAVE),
            Shoot(Down(6)),
            Idle(T_WAVE),
            Shoot(Down(7)),
            Idle(T_WAVE),
            Shoot(Down(8)),
            Idle(T_WAVE),
            Shoot(Down(9)),
            Idle(T_WAVE),
            Shoot(Down(10)),
            Idle(T_WAVE),
            Shoot(Down(11)),
            Idle(T_WAVE),
            Shoot(Down(12)),
            Idle(T_WAVE),
            DashToX(X_4), 
            FlyToXY { target: i2(X_4, Y_BOTTOM), speed: FLY_SPEED_FAST },
        ],
    };

    queue.append(&mut next);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ShotType {
    Single,
    Wall,
    PlusBurst { angle: f32 },
    Down(i32),
}

fn shoot(type_: ShotType, boss: &TestBoss, g: &mut GameData) {
    use ShotType::*;

    let c = boss.bounds().center();
    let a = if boss.dir == DirH::L { 0.5 } else { 0.0 };

    match type_ {
        Single => { 
            shoot_one(c, a, g);
        },
        Wall => { 
            shoot_one(c + i2(0, 8), a, g);
            shoot_one(c, a, g);
            shoot_one(c + i2(0, -8), a, g);
            shoot_one(c + i2(0, -16), a, g);

            shoot_one(c + i2(0, 8), a + 0.5, g);
            shoot_one(c, a + 0.5, g);
            shoot_one(c + i2(0, -8), a + 0.5, g);
            shoot_one(c + i2(0, -16), a + 0.5, g);

            let c = c + i2(0, -7 * P16.y);

            shoot_one(c + i2(0, 8), a, g);
            shoot_one(c, a, g);
            shoot_one(c + i2(0, -8), a, g);
            shoot_one(c + i2(0, -16), a, g);

            shoot_one(c + i2(0, 8), a + 0.5, g);
            shoot_one(c, a + 0.5, g);
            shoot_one(c + i2(0, -8), a + 0.5, g);
            shoot_one(c + i2(0, -16), a + 0.5, g);
        },
        PlusBurst { angle } => { 
            shoot_one(c, a + angle + 0.00, g);
            shoot_one(c, a + angle + 0.25, g);
            shoot_one(c, a + angle + 0.50, g);
            shoot_one(c, a + angle + 0.75, g);
        },
        Down(x) => {
            shoot_one(i2(x * P16.x + 8, 24), 0.25, g);
        },
    }
}

fn shoot_one(center: IVec2, angle: f32, g: &mut GameData) {
    const BULLET_SPEED: f32 = 4.0;

    let rads = 2.0 * PI * angle;
    let vel = f2(
        f32::cos(rads),
        f32::sin(rads),
    ) * BULLET_SPEED;

    let bullet = Bullet::new(false, center, vel);
    spawn_entity(bullet, g);
}