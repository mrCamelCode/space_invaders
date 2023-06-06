use rand::{thread_rng, Rng};
use thomas::{
    GameCommand, IntCoords2d, Query, System, SystemsGenerator, TerminalTransform, Timer,
    EVENT_INIT, EVENT_UPDATE,
};

use crate::{make_star, Star, StarSpawner, SCREEN_HEIGHT, SCREEN_WIDTH};

const SPAWN_WAIT_TIME: u128 = 800;
const NUM_STARS_TO_SPAWN_AT_START: u64 = 20;

pub struct StarsSystemsGenerator {}
impl SystemsGenerator for StarsSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (
                EVENT_INIT,
                System::new(vec![], |_, commands| {
                    commands
                        .borrow_mut()
                        .issue(GameCommand::AddEntity(vec![Box::new(StarSpawner {
                            spawn_timer: Timer::start_new(),
                        })]));

                    for _ in 0..NUM_STARS_TO_SPAWN_AT_START {
                        commands
                            .borrow_mut()
                            .issue(GameCommand::AddEntity(make_star(Some(IntCoords2d::new(
                                thread_rng().gen_range(0..SCREEN_WIDTH) as i64,
                                thread_rng().gen_range(0..SCREEN_HEIGHT) as i64,
                            )))));
                    }
                }),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has::<Star>().has::<TerminalTransform>()],
                    |results, _| {
                        if let [star_results, ..] = &results[..] {
                            for star_result in star_results {
                                let mut star = star_result.components().get_mut::<Star>();
                                let mut transform =
                                    star_result.components().get_mut::<TerminalTransform>();

                                if star.move_timer.elapsed_millis() >= star.move_wait_time {
                                    transform.coords += IntCoords2d::up();

                                    star.move_timer.restart();
                                }
                            }
                        }
                    },
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has::<StarSpawner>()],
                    |results, commands| {
                        if let [star_spawner_results, ..] = &results[..] {
                            let mut spawner = star_spawner_results.get_only_mut::<StarSpawner>();

                            if spawner.spawn_timer.elapsed_millis() >= SPAWN_WAIT_TIME {
                                commands
                                    .borrow_mut()
                                    .issue(GameCommand::AddEntity(make_star(None)));

                                spawner.spawn_timer.restart();
                            }
                        }
                    },
                ),
            ),
        ]
    }
}
