use thomas::core::{
    Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Message, Transform,
};
use thomas::{get_behaviour_name, Behaviour};

use crate::{make_enemy, EnemyCombatBehaviour, MSG_ENEMY_DIED, MSG_RESET, SCREEN_WIDTH};

const NUM_ENEMIES_IN_ROW: u32 = 6;
const NUM_ENEMY_ROWS: u32 = 3;

const ENEMY_PADDING: u32 = 2;

#[derive(Behaviour, Clone)]
pub struct EnemySpawnerBehaviour {
    num_remaining_enemies: u32,
    should_reset: bool,
}
impl EnemySpawnerBehaviour {
    pub fn new() -> Self {
        Self {
            num_remaining_enemies: 0,
            should_reset: false,
        }
    }

    fn destroy_remaining_enemies(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            world, commands, ..
        } = utils;

        let mut num_destroyed = 0;

        world.entities().for_each(|(entity, behaviours)| {
            if behaviours
                .has_behaviour::<EnemyCombatBehaviour>(get_behaviour_name!(EnemyCombatBehaviour))
            {
                commands.issue(GameCommand::DestroyEntity(entity.id().to_string()));

                num_destroyed += 1;
            }
        });
    }

    fn spawn_enemies(&mut self, utils: &mut BehaviourUtils) {
        for row in 0..NUM_ENEMY_ROWS {
            for col in 0..NUM_ENEMIES_IN_ROW {
                let (entity, behaviours) = make_enemy(Transform::new(Coords::new(
                    get_enemy_x_pos(SCREEN_WIDTH as u32, ENEMY_PADDING, NUM_ENEMIES_IN_ROW, col)
                        as f64,
                    1.0 + (row * ENEMY_PADDING) as f64,
                    0.0,
                )));

                utils
                    .commands
                    .issue(GameCommand::AddEntity { entity, behaviours });
            }
        }

        self.num_remaining_enemies = NUM_ENEMY_ROWS * NUM_ENEMIES_IN_ROW;
    }

    fn reset(&mut self, utils: &mut BehaviourUtils) {
        self.destroy_remaining_enemies(utils);
    }
}
impl CustomBehaviour for EnemySpawnerBehaviour {
    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.num_remaining_enemies == 0 {
            self.spawn_enemies(utils);
        }

        if self.should_reset {
            self.reset(utils);

            self.should_reset = false;
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_ENEMY_DIED => {
                if self.num_remaining_enemies > 0 {
                    self.num_remaining_enemies -= 1;
                }
            }
            MSG_RESET => {
                self.should_reset = true;
            }
            _ => (),
        }
    }
}

fn get_enemy_x_pos(total_area_width: u32, padding: u32, num_enemies_in_row: u32, col: u32) -> u32 {
    let center_cell = (total_area_width - 1) / 2;
    let enemies_width = num_enemies_in_row + (num_enemies_in_row - 1) * padding;
    let enemies_center = (enemies_width - 1) / 2;

    (center_cell - enemies_center) + (padding + 1) * col
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_enemy_x_pos {
        use super::*;

        #[test]
        fn works_with_odd_screen_width_odd_padding_and_odd_enemies() {
            assert_eq!(get_enemy_x_pos(13, 1, 5, 0), 2);
            assert_eq!(get_enemy_x_pos(13, 1, 5, 2), 6);
            assert_eq!(get_enemy_x_pos(13, 1, 5, 4), 10);
        }

        #[test]
        fn works_with_odd_screen_width_odd_padding_and_even_enemies() {
            assert_eq!(get_enemy_x_pos(5, 1, 2, 0), 1);
            assert_eq!(get_enemy_x_pos(5, 1, 2, 1), 3);
        }

        #[test]
        fn works_with_odd_screen_width_even_padding_and_odd_enemies() {
            assert_eq!(get_enemy_x_pos(7, 2, 3, 0), 0);
            assert_eq!(get_enemy_x_pos(7, 2, 3, 1), 3);
            assert_eq!(get_enemy_x_pos(7, 2, 3, 2), 6);
        }

        #[test]
        fn works_with_even_screen_width_even_padding_and_even_enemies() {
            assert_eq!(get_enemy_x_pos(6, 2, 2, 0), 1);
            assert_eq!(get_enemy_x_pos(6, 2, 2, 1), 4);
        }
    }
}
