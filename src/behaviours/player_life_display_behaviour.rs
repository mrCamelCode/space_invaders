use thomas::core::{
    Behaviour, BehaviourList, BehaviourUtils, Coords, CustomBehaviour, Entity, GameCommand, Layer,
    TerminalRenderable, Transform,
};
use thomas::{get_behaviour_name, Behaviour};

use crate::{PlayerCombatBehaviour, MAX_LIVES, PLAYER_DISPLAY_CHAR, PLAYER_ID, SCREEN_HEIGHT};

const LIFE_INDICATOR_ID_PREFIX: &str = "player-life-";

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

    fn clear_player_lives_display(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils { commands, .. } = utils;

        for life in 0..MAX_LIVES {
            commands.issue(GameCommand::DestroyEntity(
                get_life_indicator_id(life).to_string(),
            ))
        }

        self.current_lives_displayed = 0;
    }

    fn get_player_lives_display_entities(
        &mut self,
        utils: &mut BehaviourUtils,
    ) -> Vec<(Entity, BehaviourList)> {
        (0..self.get_player_lives(utils))
            .map(|life| {
                make_life_indicator(
                    get_life_indicator_id(life).as_str(),
                    Transform::new(Coords::new(
                        (0 + life) as f64,
                        (SCREEN_HEIGHT - 1) as f64,
                        0.0,
                    )),
                )
            })
            .collect::<Vec<(Entity, BehaviourList)>>()
    }
}
impl CustomBehaviour for PlayerLifeDisplayBehaviour {
    fn update(&mut self, utils: &mut BehaviourUtils) {
        let player_lives = self.get_player_lives(utils);

        if player_lives != self.current_lives_displayed {
            self.clear_player_lives_display(utils);

            for (entity, behaviours) in self.get_player_lives_display_entities(utils) {
                utils
                    .commands
                    .issue(GameCommand::AddEntity { entity, behaviours });
            }

            self.current_lives_displayed = player_lives;
        }
    }
}

fn get_life_indicator_id(life_number: u8) -> String {
    format!("{LIFE_INDICATOR_ID_PREFIX}{life_number}")
}

fn make_life_indicator(id: &str, transform: Transform) -> (Entity, BehaviourList) {
    let entity = Entity::new_with_id("life indicator", transform, id);

    (
        entity,
        BehaviourList::from(vec![Box::new(TerminalRenderable::new(
            PLAYER_DISPLAY_CHAR,
            Layer::above(&Layer::base()),
        ))]),
    )
}
