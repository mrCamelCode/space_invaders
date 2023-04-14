use thomas::{
    core::{Behaviour, Coords, CustomBehaviour, GameCommand, Timer},
    Behaviour,
};

use crate::constants::{SCREEN_HEIGHT, STAR_WAIT_TIME_MILLIS};

#[derive(Behaviour, Clone)]
pub struct StarBehaviour {
    move_timer: Timer,
}
impl StarBehaviour {
    pub fn new() -> Self {
        Self {
            move_timer: Timer::new(),
        }
    }
}
impl CustomBehaviour for StarBehaviour {
    fn init(&mut self, _: &mut thomas::core::BehaviourUtils) {
        self.move_timer.start();
    }

    fn update(&mut self, utils: &mut thomas::core::BehaviourUtils) {
        if self.move_timer.elapsed_millis() >= STAR_WAIT_TIME_MILLIS as u128 {
            utils.entity.transform_mut().move_by(&Coords::up());

            self.move_timer.restart();
        }

        if utils.entity.transform().coords().y() >= SCREEN_HEIGHT as f64 {
            utils
                .commands
                .issue(GameCommand::DestroyEntity(utils.entity.id().to_string()));
        }
    }
}
