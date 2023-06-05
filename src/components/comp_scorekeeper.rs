use thomas::Component;

#[derive(Component)]
pub struct Scorekeeper {
  pub score: u64,
  pub high_score: u64,
}