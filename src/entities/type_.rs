

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    None,
    Player,
    Enemy,
    PlayerWeapon(i32),
    EnemyWeapon(i32),
}