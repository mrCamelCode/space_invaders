use thomas::{Component, Timer};

#[derive(Component)]
pub struct StarSpawner {
  pub spawn_timer: Timer,
}

#[derive(Component)]
pub struct Star {
  pub move_timer: Timer,
  pub move_wait_time: u128,
}