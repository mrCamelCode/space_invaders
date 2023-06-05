use thomas::{
    GameCommand, IntCoords2d, Query, QueryResultList, System, SystemExtraArgs, SystemsGenerator,
    EVENT_UPDATE,
};

use crate::{make_enemy, Enemy, SCREEN_HEIGHT, SCREEN_WIDTH};

const ENEMY_GRID_WIDTH: u64 = 10;
const ENEMY_GRID_HEIGHT: u64 = 4;
const ENEMY_PADDING: u64 = 2;

pub struct EnemySystemsGenerator {}
impl SystemsGenerator for EnemySystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![(
            EVENT_UPDATE,
            System::new(vec![Query::new().has::<Enemy>()], spawn_enemies),
        )]
    }
}

fn spawn_enemies(results: Vec<QueryResultList>, util: &SystemExtraArgs) {
    if let [enemy_results, ..] = &results[..] {
        if enemy_results.len() == 0 {
            for row in 0..ENEMY_GRID_HEIGHT {
                for col in 0..ENEMY_GRID_WIDTH {
                    util.commands()
                        .issue(GameCommand::AddEntity(make_enemy(IntCoords2d::new(
                            get_enemy_x_pos(SCREEN_WIDTH, ENEMY_PADDING, ENEMY_GRID_WIDTH, col)
                                as i64,
                            row as i64,
                        ))));
                }
            }
        }
    }
}

fn get_enemy_x_pos(total_area_width: u64, padding: u64, num_enemies_in_row: u64, col: u64) -> u64 {
    let center_cell = (total_area_width - 1) / 2;
    let enemies_width = num_enemies_in_row + (num_enemies_in_row - 1) * padding;
    let enemies_center = (enemies_width - 1) / 2;

    (center_cell - enemies_center) + (padding + 1) * col
}
