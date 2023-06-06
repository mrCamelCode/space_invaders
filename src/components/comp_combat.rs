use thomas::{Component, Timer, IntCoords2d};

#[derive(Component)]
pub struct Combat {
  pub shoot_timer: Timer,
}

#[derive(Component)]
pub struct Bullet {
  pub direction: IntCoords2d,
  pub move_timer: Timer,
}