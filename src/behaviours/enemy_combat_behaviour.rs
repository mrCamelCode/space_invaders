use thomas::core::{Behaviour, CustomBehaviour, Message};
use thomas::Behaviour;

use crate::MSG_BULLET_HIT;

#[derive(Behaviour, Clone)]
pub struct EnemyCombatBehaviour {
    health: u32,
}
impl EnemyCombatBehaviour {
    pub fn new() -> Self {
        Self { health: 1 }
    }
}
impl CustomBehaviour for EnemyCombatBehaviour {
    fn update(&mut self, utils: &mut thomas::core::BehaviourUtils) {
        if self.health <= 0 {
          utils.entity.destroy();
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_BULLET_HIT => {
                self.health = 0;
            }
            _ => (),
        }
    }
}
