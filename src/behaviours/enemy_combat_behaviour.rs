use std::ops::RangeInclusive;

use rand::{thread_rng, Rng};
use thomas::core::{
    Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Message, Timer, Transform,
};
use thomas::Behaviour;

use crate::{
    make_bullet, BulletHitPayload, BulletType, ENEMY_SPAWNER_ID, MSG_BULLET_HIT, MSG_ENEMY_DIED,
};

const CHANCE_TO_SHOOT: u8 = 50;
const SHOOT_ROLL_WAIT_TIME_RANGE_MILLIS: RangeInclusive<u128> = 1500..=2500;

#[derive(Behaviour, Clone)]
pub struct EnemyCombatBehaviour {
    health: u32,
    shoot_wait_timer: Timer,
    shoot_roll_wait_time: u128,
}
impl EnemyCombatBehaviour {
    pub fn new() -> Self {
        Self {
            health: 1,
            shoot_wait_timer: Timer::new(),
            shoot_roll_wait_time: thread_rng().gen_range(SHOOT_ROLL_WAIT_TIME_RANGE_MILLIS),
        }
    }

    fn should_shoot(&self) -> bool {
        let roll = thread_rng().gen_range(1..=100);

        roll <= CHANCE_TO_SHOOT
    }

    fn shoot(&self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            commands, entity, ..
        } = utils;

        let (entity, behaviours) = make_bullet(
            Transform::new(*entity.transform().coords() + Coords::up()),
            BulletType::Enemy,
            Coords::up(),
        );

        commands.issue(GameCommand::AddEntity { entity, behaviours })
    }
}
impl CustomBehaviour for EnemyCombatBehaviour {
    fn init(&mut self, _: &mut BehaviourUtils) {
        self.shoot_wait_timer.start();
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.health <= 0 {
            utils
                .commands
                .issue(GameCommand::DestroyEntity(utils.entity.id().to_string()));
        }

        if self.shoot_wait_timer.elapsed_millis() >= self.shoot_roll_wait_time {
            if self.should_shoot() {
                self.shoot(utils);
            }

            self.shoot_wait_timer.restart();
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_BULLET_HIT => {
                if let Some(payload) = Message::<BulletHitPayload>::get_payload(message) {
                    if payload.bullet_type == BulletType::Player {
                        self.health = 0;
                    }
                }
            }
            _ => (),
        }
    }

    fn on_destroy(&mut self, utils: &mut BehaviourUtils) {
        utils.commands.issue(GameCommand::SendMessage {
            entity_id: ENEMY_SPAWNER_ID.to_string(),
            message: Message::new(MSG_ENEMY_DIED, Box::new(0)),
        })
    }
}
