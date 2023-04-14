use rand::{thread_rng, Rng};
use thomas::{
    core::{Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Timer, Transform},
    Behaviour,
};

use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH, STAR_WAIT_TIME_MILLIS},
    entity_generators::make_star,
};

#[derive(Behaviour, Clone)]
pub struct StarSpawnerBehaviour {
    spawn_timer: Timer,
}
impl StarSpawnerBehaviour {
    pub fn new() -> Self {
        Self {
            spawn_timer: Timer::new(),
        }
    }
}
impl CustomBehaviour for StarSpawnerBehaviour {
    fn init(&mut self, utils: &mut BehaviourUtils) {
        for col in (0..SCREEN_WIDTH).step_by(3) {
            let (entity, behaviours) = make_star(Transform::new(Coords::new(
                col as f64,
                thread_rng().gen_range(0..SCREEN_HEIGHT as u16) as f64,
                0.0,
            )));

            utils
                .commands
                .issue(GameCommand::AddEntity { entity, behaviours });
        }

        self.spawn_timer.start();
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.spawn_timer.elapsed_millis() > STAR_WAIT_TIME_MILLIS {
            let (entity, behaviours) = make_star(Transform::new(Coords::new(
                thread_rng().gen_range(0..SCREEN_WIDTH as u16) as f64,
                0.0,
                0.0,
            )));

            utils
                .commands
                .issue(GameCommand::AddEntity { entity, behaviours });

            self.spawn_timer.restart();
        }
    }
}
