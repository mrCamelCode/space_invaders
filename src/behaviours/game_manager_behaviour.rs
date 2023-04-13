use thomas::core::{
    Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Message, Transform,
};
use thomas::{get_behaviour_name, Behaviour};

use crate::{
    make_text, ChangeTextPayload, PlayerCombatBehaviour, ENEMY_SPAWNER_ID, MSG_CHANGE_TEXT,
    MSG_ENEMY_DIED, MSG_PLAYER_KILLED_ENEMY, MSG_RESET, PLAYER_ID, SCREEN_HEIGHT,
};

const ENEMY_KILL_SCORE: u32 = 10;

const SCORE_TEXT_ID: &str = "player-score-text";

#[derive(Behaviour, Clone)]
pub struct GameManagerBehaviour {
    score: u32,
    score_addition: u32,
}
impl GameManagerBehaviour {
    pub fn new() -> Self {
        Self {
            score: 0,
            score_addition: 0,
        }
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

    fn reset_game(&mut self, utils: &mut BehaviourUtils) {
        self.score = 0;
        self.score_addition = 0;

        self.update_score(utils);

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

    fn update_score(&mut self, utils: &mut BehaviourUtils) {
        self.score += self.score_addition;
        self.score_addition = 0;

        utils.commands.issue(GameCommand::SendMessage {
            entity_id: SCORE_TEXT_ID.to_string(),
            message: Message::new(
                MSG_CHANGE_TEXT,
                Box::new(ChangeTextPayload {
                    new_text: format!("Score: {}", self.score),
                }),
            ),
        });
    }
}
impl CustomBehaviour for GameManagerBehaviour {
    fn init(&mut self, utils: &mut BehaviourUtils) {
        let (entity, behaviours) = make_text(
            Transform::new(Coords::new(0.0, 0.0, 0.0)),
            SCORE_TEXT_ID,
            &format!("Score: {}", self.score),
        );

        utils
            .commands
            .issue(GameCommand::AddEntity { entity, behaviours });
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        if self.should_reset(utils) {
            self.reset_game(utils);
        }

        if self.score_addition > 0 {
            self.update_score(utils);
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_PLAYER_KILLED_ENEMY => {
                self.score_addition += ENEMY_KILL_SCORE;
            }
            _ => (),
        }
    }
}
