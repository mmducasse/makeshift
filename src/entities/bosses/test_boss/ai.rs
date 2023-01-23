use std::f32::consts::PI;

use xf::{data::{spin::Spin, dir_h::DirH}, num::{ivec2::{IVec2, i2}, fvec2::f2}, time::{countdown::Countdown}};

use crate::{
    game::game_data::GameData, 
    entities::{entity::Entity, bosses::test_boss::state, spawn::spawn_entity, bullets::bullet::Bullet}, 
    consts::{P16, P8}
};

use super::test_boss::TestBoss;

const T_IDLE: u32 = 30;

const X_POINT_1: i32 = 3 * P16.x;
const X_POINT_2: i32 = (6 * P16.x) + P8.x;
const X_POINT_3: i32 = 11 * P16.x;

const CENTER_POINT: IVec2 = i2(X_POINT_2, 5 * P16.y);

#[derive(Clone, Copy, PartialEq, Debug)]
enum AiState {
    Idle(u32),
    RunToX(i32),
    DashToX(i32),
    FlyToXY(IVec2),
    Float(u32),
    Shoot(ShotType),
    Whirl(Spin, u32),
    FloorCeil(u32),
}

impl AiState {
    pub fn time(self) -> u32 {
        use AiState::*;

        match self {
            Idle(t) | Whirl(_, t) | FloorCeil(t) | Float(t) => t,
            _ => 0,
        }
    }
}

pub struct Ai {
    state: AiState,
    state_timer: Countdown,
    queue: Vec<AiState>,
}

impl Ai {
    pub fn new() -> Self {
        Self {
            state: AiState::Idle(0),
            state_timer: Countdown::new(0),
            queue: vec![AiState::Idle(15)],
        }
    }

    pub fn update(boss: &mut TestBoss, g: &mut GameData) {
        use AiState::*;

        if !boss.grace_timer.is_done() {
            reset_ai(boss);
            return;
        }

        boss.ai.state_timer.decrement();
        
        if is_state_done(boss) {
            if boss.ai.queue.is_empty() {
                add_states(&mut boss.ai.queue, g.frame_num());
            }
            
            let state = boss.ai.queue.remove(0);
            boss.ai.state = state;
            boss.ai.state_timer = Countdown::new(boss.ai.state.time());

            println!("New state = {:?}", boss.ai.state);
        }

        match boss.ai.state {
            Idle(_) => {
                boss.state = state::State::Idle;
            },
            RunToX(x) => {
                boss.state = state::State::Run;
                boss.face_toward(x);
            },
            DashToX(x) => {
                boss.state = state::State::Dash;
                boss.face_toward(x);
            },
            FlyToXY(pos) => {
                boss.state = state::State::FlyTo(pos);
            },
            Float(_) => {
                boss.state = state::State::Float;
            },
            // Whirl(_, _) => todo!(),
            // FloorCeil(_) => todo!(),
            _ => { todo!() }
        }
    }
}

fn is_state_done(boss: &mut TestBoss) -> bool {
    use AiState::*;

    match boss.ai.state {
        Idle(_) | Whirl(_, _) | FloorCeil(_) | Float(_) => {
            boss.ai.state_timer.is_done()
        },
        RunToX(x) | DashToX(x) => {
            let target = i2(x, boss.bounds().center().y);
            boss.bounds().contains(target)
        },
        FlyToXY(target) => {
            boss.bounds().contains(target)
        },
        _ => true,
    }
}

fn reset_ai(boss: &mut TestBoss) {
    boss.ai.state = AiState::Idle(0);
    boss.ai.state_timer = Countdown::new(0);
    boss.ai.queue.clear();
}

fn add_states(queue: &mut Vec<AiState>, seed: usize) {
    use AiState::*;

    let mut next = match seed % 2 {
        _ => vec![RunToX(X_POINT_1), RunToX(X_POINT_2), FlyToXY(CENTER_POINT), Float(120), Idle(15)]
        // 0 => vec![RunToX(X_POINT_2), DashToX(X_POINT_1), RunToX(X_POINT_2), DashToX(X_POINT_3)],
        // 1 => vec![DashToX(X_POINT_1), Idle(T_IDLE), DashToX(X_POINT_3), Idle(T_IDLE)],
        // 2 => vec![DashToX(X_POINT_1), DashToX(X_POINT_2), DashToX(X_POINT_1), DashToX(X_POINT_3), DashToX(X_POINT_2), DashToX(X_POINT_3)],
        // _ => vec![RunToX(X_POINT_1), Idle(T_IDLE), RunToX(X_POINT_3), Idle(T_IDLE)],
    };

    queue.append(&mut next);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ShotType {
    Single,
    Wall,
    PlusBurst { angle: f32 },
}

fn shoot(type_: ShotType, boss: &TestBoss, g: &mut GameData) {
    use ShotType::*;

    let c = boss.bounds().center();
    let a = if boss.dir == DirH::L { 0.5 } else { 0.0 };

    match type_ {
        Single => { shoot_one(c, a, g) },
        Wall => todo!(),
        PlusBurst { angle } => todo!(),
    }
}

fn shoot_one(center: IVec2, angle: f32, g: &mut GameData) {
    const BULLET_SPEED: f32 = 2.0;

    let rads = 2.0 * PI * angle;
    let vel = f2(
        f32::cos(rads),
        f32::sin(rads),
    ) * BULLET_SPEED;

    let bullet = Bullet::new(false, center, vel);
    spawn_entity(bullet, g);
}