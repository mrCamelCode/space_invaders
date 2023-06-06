use thomas::{Component, Timer};

#[derive(Component)]
pub struct Enemy {
  pub shoot_timer: Timer,
}

#[derive(Component)]
pub struct EnemyMovement {
  pub move_timer: Timer,
}