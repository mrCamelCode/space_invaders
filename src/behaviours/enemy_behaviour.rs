use thomas::{
    core::{Behaviour, BehaviourUtils, CustomBehaviour, GameCommand, Message},
    Behaviour,
};

use crate::{ENEMY_SPAWNER_ID, MSG_ENEMY_DIED};

#[derive(Behaviour, Clone)]
pub struct EnemyBehaviour {}
impl EnemyBehaviour {
    pub fn new() -> Self {
        Self {}
    }
}
impl CustomBehaviour for EnemyBehaviour {
    fn on_destroy(&mut self, utils: &mut BehaviourUtils) {
        utils.commands.issue(GameCommand::SendMessage {
            entity_id: ENEMY_SPAWNER_ID.to_string(),
            message: Message::new(MSG_ENEMY_DIED, Box::new(0)),
        })
    }
}
