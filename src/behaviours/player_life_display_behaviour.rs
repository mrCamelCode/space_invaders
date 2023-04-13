use thomas::core::{
    Behaviour, BehaviourUtils, Coords, CustomBehaviour, GameCommand, Message, Transform,
};
use thomas::{get_behaviour_name, Behaviour};

use crate::{
    make_text, ChangeTextPayload, PlayerCombatBehaviour, MSG_CHANGE_TEXT, PLAYER_DISPLAY_CHAR,
    PLAYER_ID, SCREEN_HEIGHT,
};

const PLAYER_LIVES_TEXT_ID: &str = "player-lives-text";

#[derive(Behaviour, Clone)]
pub struct PlayerLifeDisplayBehaviour {
    current_lives_displayed: u8,
}
impl PlayerLifeDisplayBehaviour {
    pub fn new() -> Self {
        Self {
            current_lives_displayed: 0,
        }
    }

    fn get_player_lives(&self, utils: &mut BehaviourUtils) -> u8 {
        let BehaviourUtils { world, .. } = utils;

        if let Some((_, player_behaviours)) = world.get_entity(PLAYER_ID) {
            if let Some(player_combat_behaviour) = player_behaviours
                .get_behaviour::<PlayerCombatBehaviour>(get_behaviour_name!(PlayerCombatBehaviour))
            {
                return player_combat_behaviour.lives();
            }
        }

        0
    }
}
impl CustomBehaviour for PlayerLifeDisplayBehaviour {
    fn init(&mut self, utils: &mut BehaviourUtils) {
        let (entity, behaviours) = make_text(
            Transform::new(Coords::new(0.0, (SCREEN_HEIGHT - 1) as f64, 0.0)),
            PLAYER_LIVES_TEXT_ID,
            "Lives:",
        );

        utils
            .commands
            .issue(GameCommand::AddEntity { entity, behaviours });
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        let player_lives = self.get_player_lives(utils);

        let BehaviourUtils { commands, .. } = utils;

        if player_lives != self.current_lives_displayed {
            commands.issue(GameCommand::SendMessage {
                entity_id: PLAYER_LIVES_TEXT_ID.to_string(),
                message: Message::new(
                    MSG_CHANGE_TEXT,
                    Box::new(ChangeTextPayload {
                        new_text: format!(
                            "Lives: {}",
                            (0..player_lives)
                                .map(|_| PLAYER_DISPLAY_CHAR.to_string())
                                .collect::<Vec<String>>()
                                .join("")
                        ),
                    }),
                ),
            });

            self.current_lives_displayed = player_lives;
        }
    }
}
