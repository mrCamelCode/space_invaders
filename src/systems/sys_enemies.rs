use rand::prelude::*;
use thomas::{
    GameCommand, GameCommandsArg, IntCoords2d, Query, QueryResultList, System, SystemsGenerator,
    TerminalTransform, Timer, EVENT_INIT, EVENT_UPDATE,
};

use crate::{
    make_bullet, make_enemy, Bullet, BulletType, Enemy, EnemyMovement, Scorekeeper, SCREEN_WIDTH,
};

const ENEMY_GRID_WIDTH: u64 = 10;
const ENEMY_GRID_HEIGHT: u64 = 4;
const ENEMY_PADDING: u64 = 2;

const ENEMY_SHOOT_WAIT_TIME: u128 = 200;
const ENEMY_SHOOT_CHANCE: u8 = 10;

const ENEMY_MOVE_WAIT_TIME: u128 = 800;
const ENEMY_MIN_X: u64 = 3;
const ENEMY_MAX_X: u64 = SCREEN_WIDTH - 4;

pub struct EnemySystemsGenerator {}
impl SystemsGenerator for EnemySystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (
                EVENT_INIT,
                System::new(vec![], |_, commands| {
                    commands
                        .borrow_mut()
                        .issue(GameCommand::AddEntity(vec![Box::new(EnemyMovement {
                            move_timer: Timer::start_new(),
                        })]))
                }),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Enemy>(),
                        Query::new().has::<Bullet>(),
                        Query::new().has::<Scorekeeper>(),
                    ],
                    spawn_enemies,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has::<Enemy>().has::<TerminalTransform>()],
                    shoot,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Enemy>().has::<TerminalTransform>(),
                        Query::new().has::<EnemyMovement>(),
                        Query::new()
                            .has::<Enemy>()
                            .has_where::<TerminalTransform>(|transform| {
                                transform.coords.x() <= ENEMY_MIN_X as i64
                            }),
                        Query::new()
                            .has::<Enemy>()
                            .has_where::<TerminalTransform>(|transform| {
                                transform.coords.x() >= ENEMY_MAX_X as i64
                            }),
                    ],
                    movement,
                ),
            ),
        ]
    }
}

fn spawn_enemies(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [enemy_results, bullet_results, scorekeeper_results, ..] = &results[..] {
        if enemy_results.len() == 0 {
            for row in 0..ENEMY_GRID_HEIGHT {
                for col in 0..ENEMY_GRID_WIDTH {
                    commands
                        .borrow_mut()
                        .issue(GameCommand::AddEntity(make_enemy(IntCoords2d::new(
                            get_enemy_x_pos(SCREEN_WIDTH, ENEMY_PADDING, ENEMY_GRID_WIDTH, col)
                                as i64,
                            row as i64,
                        ))));
                }
            }

            for bullet_result in bullet_results {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*bullet_result.entity()));
            }

            scorekeeper_results.get_only_mut::<Scorekeeper>().level += 1;
        }
    }
}

fn get_enemy_x_pos(total_area_width: u64, padding: u64, num_enemies_in_row: u64, col: u64) -> u64 {
    let center_cell = (total_area_width - 1) / 2;
    let enemies_width = num_enemies_in_row + (num_enemies_in_row - 1) * padding;
    let enemies_center = (enemies_width - 1) / 2;

    (center_cell - enemies_center) + (padding + 1) * col
}

fn shoot(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [enemy_results, ..] = &results[..] {
        for enemy_result in enemy_results {
            let mut enemy = enemy_result.components().get_mut::<Enemy>();
            let transform = enemy_result.components().get::<TerminalTransform>();

            if enemy.shoot_timer.elapsed_millis() >= ENEMY_SHOOT_WAIT_TIME {
                let shoot_roll: u8 = thread_rng().gen_range(1..=100);

                if shoot_roll <= ENEMY_SHOOT_CHANCE {
                    commands
                        .borrow_mut()
                        .issue(GameCommand::AddEntity(make_bullet(
                            transform.coords + IntCoords2d::up(),
                            IntCoords2d::up(),
                            BulletType::Enemy,
                        )));
                }

                enemy.shoot_timer.restart();
            }
        }
    }
}

fn movement(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [enemy_results, enemy_movement_results, enemies_at_min_x_results, enemies_at_max_x_results, ..] =
        &results[..]
    {
        let mut enemy_movement = enemy_movement_results.get_only_mut::<EnemyMovement>();

        if enemy_movement.move_timer.elapsed_millis() >= ENEMY_MOVE_WAIT_TIME {
            let direction = if enemies_at_min_x_results.len() > 0 {
                IntCoords2d::right()
            } else if enemies_at_max_x_results.len() > 0 {
                IntCoords2d::left()
            } else {
                let direction_roll: u8 = thread_rng().gen_range(0..=1);
                match direction_roll {
                    0 => IntCoords2d::left(),
                    _ => IntCoords2d::right(),
                }
            };

            for enemy_result in enemy_results {
                let mut transform = enemy_result.components().get_mut::<TerminalTransform>();

                transform.coords += direction;
            }

            enemy_movement.move_timer.restart();
        }
    }
}
