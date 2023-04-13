use thomas::core::{Behaviour, BehaviourUtils, CustomBehaviour, GameCommand, Message};
use thomas::{get_behaviour_name, Behaviour};

use crate::{PlayerCombatBehaviour, ENEMY_SPAWNER_ID, MSG_RESET, PLAYER_ID};

#[derive(Behaviour, Clone)]
pub struct GameManagerBehaviour {}
impl GameManagerBehaviour {
    pub fn new() -> Self {
        Self {}
    }

    fn should_reset(&self, utils: &mut BehaviourUtils) -> bool {
        let BehaviourUtils { world, .. } = utils;

        if let Some((_, player_behaviours)) = world.get_entity(PLAYER_ID) {
            if let Some(combat_behaviour) = player_behaviours
                .get_behaviour::<PlayerCombatBehaviour>(get_behaviour_name!(PlayerCombatBehaviour))
            {
                return combat_behaviour.lives() == 0;
            }
        }

        false
    }

    fn reset_game(&self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            world, commands, ..
        } = utils;

        let (enemy_spawner, _) = world
            .get_entity(ENEMY_SPAWNER_ID)
            .expect("Enemy Spawner should always exist.");
        let (player, _) = world
            .get_entity(PLAYER_ID)
            .expect("Player should always exist.");

        commands.issue(GameCommand::SendMessage {
            entity_id: enemy_spawner.id().to_string(),
            message: Message::new(MSG_RESET, Box::new(0)),
        });
        commands.issue(GameCommand::SendMessage {
            entity_id: player.id().to_string(),
            message: Message::new(MSG_RESET, Box::new(0)),
        });
    }
}
impl CustomBehaviour for GameManagerBehaviour {
    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.should_reset(utils) {
            self.reset_game(utils);
        }
    }
}
