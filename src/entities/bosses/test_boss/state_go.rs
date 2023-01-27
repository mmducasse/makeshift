
use xf::{data::dir_h::DirH, num::ivec2::IVec2};

use crate::{
    systems::collision::{collide, get_colliders_near}, 
    consts::GRAVITY, 
    game::game_data::GameData, 
    entities::entity::Entity,
};

use super::{
    test_boss::TestBoss, 
    state::{State, GoType}, 
    state_util::check_collide_enemy, 
    consts::{RUN_SPEED_X, DASH_SPEED_X, FLY_SPEED_SLOW, FLY_SPEED_FAST}
};

pub fn update(
    boss: &mut TestBoss,
    type_: GoType, 
    from: IVec2, 
    to_x: i32, 
    to_y: Option<i32>,
    start_t: u64,
    g: &mut GameData
) {
    let speed = match type_ {
        GoType::Run => RUN_SPEED_X,
        GoType::Dash => DASH_SPEED_X,
        GoType::Fly(speed) => speed,
    };

    let dt = g.frame_num() - start_t;

    let dx = i32::signum(to_x - from.x) as f32 * speed;
    boss.d.pos.x = from.x as f32 + dx;
    boss.dir = DirH::from_x(dx as i32);

    if let Some(to_y) = to_y {
        let dy = i32::signum(to_y - from.y) as f32 * speed;
        boss.d.pos.y = from.y as f32 + dy;
    };
    
    if type_ == GoType::Run {
        boss.d.vel.y += GRAVITY.y;
        boss.d.pos.y += boss.d.vel.y;
    }

    let colliders = get_colliders_near(boss.bounds().center(), g);
    let deflection = collide(boss.bounds(), colliders, Some(g.level.bounds()));
    boss.d.pos += deflection.as_fvec2();

    if boss.d.vel.y > 0.0 && deflection.y < 0 {
        boss.d.vel.y = 0.0;
    }

    check_collide_enemy(boss, g);
}