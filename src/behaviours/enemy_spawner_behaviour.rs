use rand::{thread_rng, Rng};
use thomas::core::{
    Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Message, Timer, Transform,
    World,
};
use thomas::{get_behaviour_name, Behaviour};

use crate::{
    make_enemy, EnemyCombatBehaviour, EnemyDiedPayload, MoveEnemyPayload, MSG_ENEMY_DIED,
    MSG_MOVE_ENEMY, MSG_RESET, SCREEN_WIDTH,
};

const ENEMY_MOVE_WAIT_TIME_MILLIS: u128 = 1500;

const NUM_ENEMIES_IN_ROW: u32 = 12;
const NUM_ENEMY_ROWS: u32 = 3;

const ENEMY_PADDING: u32 = 2;
const SCREEN_EDGE_PADDING: u32 = 4;

#[derive(Behaviour, Clone)]
pub struct EnemySpawnerBehaviour {
    should_reset: bool,
    enemy_move_timer: Timer,
    existing_enemy_entity_ids: Vec<String>,
}
impl EnemySpawnerBehaviour {
    pub fn new() -> Self {
        Self {
            should_reset: false,
            enemy_move_timer: Timer::new(),
            existing_enemy_entity_ids: vec![],
        }
    }

    fn num_remaining_enemies(&self) -> usize {
        self.existing_enemy_entity_ids.len()
    }

    fn destroy_remaining_enemies(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            world, commands, ..
        } = utils;

        world.entities().for_each(|(entity, behaviours)| {
            if behaviours
                .has_behaviour::<EnemyCombatBehaviour>(get_behaviour_name!(EnemyCombatBehaviour))
            {
                commands.issue(GameCommand::DestroyEntity(entity.id().to_string()));
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

                self.existing_enemy_entity_ids.push(entity.id().to_string());

                utils
                    .commands
                    .issue(GameCommand::AddEntity { entity, behaviours });
            }
        }
    }

    fn move_enemies(&self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            world, commands, ..
        } = utils;

        let can_move_right = if let Some(coords) = get_rightmost_enemy_pos(world) {
            coords.x() < (SCREEN_WIDTH - 1 - SCREEN_EDGE_PADDING as u16) as f64
        } else {
            false
        };
        let can_move_left = if let Some(coords) = get_leftmost_enemy_pos(world) {
            coords.x() > SCREEN_EDGE_PADDING as f64
        } else {
            false
        };

        let displacement: Coords = if thread_rng().gen_range(0..=1 as u8) == 0 && can_move_right {
            Coords::right()
        } else if can_move_left {
            Coords::left()
        } else {
            Coords::zero()
        };

        if displacement != Coords::zero() {
            for enemy_id in self.existing_enemy_entity_ids.iter() {
                commands.issue(GameCommand::SendMessage {
                    entity_id: enemy_id.to_string(),
                    message: Message::new(
                        MSG_MOVE_ENEMY,
                        Box::new(MoveEnemyPayload { displacement }),
                    ),
                });
            }
        }
    }

    fn reset(&mut self, utils: &mut BehaviourUtils) {
        self.destroy_remaining_enemies(utils);
        self.enemy_move_timer.restart();
    }
}
impl CustomBehaviour for EnemySpawnerBehaviour {
    fn init(&mut self, _: &mut BehaviourUtils) {
        self.enemy_move_timer.start();
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.should_reset {
            self.reset(utils);

            self.should_reset = false;
        }

        if self.num_remaining_enemies() == 0 {
            self.spawn_enemies(utils);
        }

        if self.enemy_move_timer.elapsed_millis() >= ENEMY_MOVE_WAIT_TIME_MILLIS {
            self.move_enemies(utils);

            self.enemy_move_timer.restart();
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_ENEMY_DIED => {
                if let Some(payload) = Message::<EnemyDiedPayload>::get_payload(message) {
                    self.existing_enemy_entity_ids
                        .retain(|id| *id != payload.enemy_id);
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

fn get_rightmost_enemy_pos(world: &World) -> Option<Coords> {
    if let Some((entity, ..)) = world
        .entities_with_behaviour::<EnemyCombatBehaviour>(get_behaviour_name!(EnemyCombatBehaviour))
        .max_by(|(entity, ..), (other_entity, ..)| {
            entity
                .transform()
                .coords()
                .x()
                .total_cmp(&other_entity.transform().coords().x())
        })
    {
        return Some(entity.transform().coords().clone());
    }

    None
}

fn get_leftmost_enemy_pos(world: &World) -> Option<Coords> {
    if let Some((entity, ..)) = world
        .entities_with_behaviour::<EnemyCombatBehaviour>(get_behaviour_name!(EnemyCombatBehaviour))
        .min_by(|(entity, ..), (other_entity, ..)| {
            entity
                .transform()
                .coords()
                .x()
                .total_cmp(&other_entity.transform().coords().x())
        })
    {
        return Some(entity.transform().coords().clone());
    }

    None
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
