

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    None,
    Player,
    Enemy(i32),
    PlayerWeapon(i32),
    EnemyWeapon(i32),
}