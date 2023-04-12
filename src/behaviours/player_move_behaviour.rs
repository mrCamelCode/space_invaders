use thomas::core::{Behaviour, BehaviourUtils, Coords, CustomBehaviour, Timer};
use thomas::Behaviour;
use thomas::Keycode;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

const MOVE_WAIT_TIME_MILLIS: u128 = 100;

const MOVE_LEFT_KEY: Keycode = Keycode::A;
const MOVE_RIGHT_KEY: Keycode = Keycode::D;

const PLAYER_Y_OFFSET: u8 = 2;

#[derive(Behaviour, Clone)]
pub struct PlayerMoveBehaviour {
    move_timer: Timer,
}
impl PlayerMoveBehaviour {
    pub fn new() -> Self {
        Self {
            move_timer: Timer::new(),
        }
    }

    pub fn perform_movement(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            services, entity, ..
        } = utils;

        let movement_keys = vec![MOVE_LEFT_KEY, MOVE_RIGHT_KEY];

        if self.move_timer.elapsed_millis() > MOVE_WAIT_TIME_MILLIS
            && movement_keys
                .iter()
                .any(|keycode| services.input().is_key_pressed(keycode))
        {
            let mut move_direction = Coords::zero();

            if !self.move_timer.is_running() {
                self.move_timer.restart();
            }

            if self.move_timer.elapsed_millis() >= MOVE_WAIT_TIME_MILLIS {
                if services.input().is_key_pressed(&MOVE_LEFT_KEY) {
                    move_direction = Coords::left();
                } else if services.input().is_key_pressed(&MOVE_RIGHT_KEY) {
                    move_direction = Coords::right();
                }

                entity.transform_mut().move_by(&move_direction);
                self.move_timer.restart();
            }
        }
    }

    pub fn clamp_position(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils { entity, .. } = utils;

        let current_coords = entity.transform().coords().clone();

        if current_coords.x() < 0.0 {
            entity.transform_mut().move_to(&Coords::new(
                0.0,
                current_coords.y(),
                current_coords.z(),
            ));
        } else if current_coords.x() >= SCREEN_WIDTH as f64 {
            entity.transform_mut().move_to(&Coords::new(
                (SCREEN_WIDTH - 1) as f64,
                current_coords.y(),
                current_coords.z(),
            ));
        }
    }
}
impl CustomBehaviour for PlayerMoveBehaviour {
    fn init(&mut self, utils: &mut BehaviourUtils) {
        utils.entity.transform_mut().move_to(&Coords::new(
            SCREEN_WIDTH as f64 / 2.0,
            SCREEN_HEIGHT as f64 - PLAYER_Y_OFFSET as f64,
            0.0,
        ));

        self.move_timer.start();
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        self.perform_movement(utils);

        self.clamp_position(utils);
    }
}
