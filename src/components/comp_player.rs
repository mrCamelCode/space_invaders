use thomas::{Component, Timer};

#[derive(Component)]
pub struct Player {
  pub lives: u8,
}

#[derive(Component)]
pub struct PlayerMovement {
  pub move_timer: Timer,
}
