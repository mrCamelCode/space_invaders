use thomas::core::{Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Transform};
use thomas::Behaviour;

use crate::{make_enemy, SCREEN_WIDTH};

const ENEMY_BLOCK_WIDTH: u32 = 4;
const ENEMY_BLOCK_HEIGHT: u32 = 4;

#[derive(Behaviour, Clone)]
pub struct EnemySpawnerBehaviour {
    num_remaining_enemies: u32,
}
impl EnemySpawnerBehaviour {
    pub fn new() -> Self {
        Self {
            num_remaining_enemies: 0,
        }
    }

    fn spawn_enemies(&mut self, utils: &mut BehaviourUtils) {
        let starting_pos = Coords::new(
            (SCREEN_WIDTH / 2) as f64 - (ENEMY_BLOCK_WIDTH + 1) as f64,
            1.0,
            0.0,
        );

        for row in 0..ENEMY_BLOCK_HEIGHT {
            for col in 0..ENEMY_BLOCK_WIDTH {
                let (entity, behaviours) = make_enemy(Transform::new(
                    starting_pos + Coords::new((col + (2 * col)) as f64, (row * 2) as f64, 0.0),
                ));

                utils
                    .commands
                    .issue(GameCommand::AddEntity { entity, behaviours });
            }
        }

        self.num_remaining_enemies = ENEMY_BLOCK_HEIGHT * ENEMY_BLOCK_WIDTH;
    }
}
impl CustomBehaviour for EnemySpawnerBehaviour {
    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.num_remaining_enemies <= 0 {
            self.spawn_enemies(utils);
        }
    }
}
