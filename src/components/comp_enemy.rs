use thomas::{Component, Timer};

#[derive(Component)]
pub struct Enemy {
  pub shoot_timer: Timer,
}