use crate::{make_bullet, BulletHitPayload, BulletType, MSG_BULLET_HIT, MSG_RESET};

use thomas::core::{
    Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Message, Timer, Transform,
};
use thomas::{Behaviour, Keycode};

const MIN_SHOOT_WAIT_TIME_MILLIS: u128 = 250;

pub const MAX_LIVES: u8 = 3;
const SHOOT_KEY: Keycode = Keycode::Space;

#[derive(Behaviour, Clone)]
pub struct PlayerCombatBehaviour {
    lives: u8,
    fire_timer: Timer,
    should_reset: bool,
}
impl PlayerCombatBehaviour {
    pub fn new() -> Self {
        Self {
            lives: MAX_LIVES,
            fire_timer: Timer::new(),
            should_reset: false,
        }
    }

    pub fn lives(&self) -> u8 {
        self.lives
    }
}
impl CustomBehaviour for PlayerCombatBehaviour {
    fn init(&mut self, _: &mut BehaviourUtils) {
        self.lives = MAX_LIVES;

        self.fire_timer.restart();
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            entity,
            services,
            commands,
            ..
        } = utils;

        if services.input().is_key_pressed(&SHOOT_KEY)
            && self.fire_timer.elapsed_millis() >= MIN_SHOOT_WAIT_TIME_MILLIS
        {
            let (bullet_entity, bullet_behaviours) = make_bullet(
                Transform::new(*entity.transform().coords() + Coords::down()),
                BulletType::Player,
                Coords::down(),
            );

            commands.issue(GameCommand::AddEntity {
                entity: bullet_entity,
                behaviours: bullet_behaviours,
            });

            self.fire_timer.restart();
        }

        if self.should_reset {
            self.init(utils);

            self.should_reset = false;
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_BULLET_HIT => {
                if let Some(payload) = Message::<BulletHitPayload>::get_payload(message) {
                    if payload.bullet_type == BulletType::Enemy && self.lives > 0 {
                        self.lives -= 1;
                    }
                }
            }
            MSG_RESET => {
                self.should_reset = true;
            }
            _ => (),
        }
    }
}
