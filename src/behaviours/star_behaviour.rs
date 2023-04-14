use thomas::{
    core::{Behaviour, Coords, CustomBehaviour, Timer},
    Behaviour,
};

use crate::constants::STAR_WAIT_TIME_MILLIS;



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
    fn init(&mut self, utils: &mut thomas::core::BehaviourUtils) {
        self.move_timer.start();
    }

    fn update(&mut self, utils: &mut thomas::core::BehaviourUtils) {
        if self.move_timer.elapsed_millis() >= STAR_WAIT_TIME_MILLIS as u128 {
            utils.entity.transform_mut().move_by(&Coords::up());

            self.move_timer.restart();
        }
    }
}
