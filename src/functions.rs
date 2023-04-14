use thomas::core::{
    BehaviourList, Entity, Game, GameConfig, Layer, TerminalRenderable, TerminalRenderer,
    TerminalRendererConfig,
};
use thomas::core::{Dimensions2d, Transform};

use crate::behaviours::StarSpawnerBehaviour;
use crate::{
    EnemySpawnerBehaviour, GameManagerBehaviour, PlayerCombatBehaviour, PlayerLifeDisplayBehaviour,
    PlayerMoveBehaviour, ENEMY_SPAWNER_ID, GAME_MANAGER_ID, PLAYER_DISPLAY_CHAR, PLAYER_ID,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub fn run() {
    let mut game = Game::new(GameConfig {
        press_escape_to_quit: true,
        max_frame_rate: 15,
    });

    game.add_entity(
        Entity::new_with_id("Game Manager", Transform::default(), GAME_MANAGER_ID),
        BehaviourList::from(vec![Box::new(GameManagerBehaviour::new())]),
    );

    game.add_entity(
        Entity::new_with_id("Player", Transform::default(), PLAYER_ID),
        BehaviourList::from(vec![
            Box::new(TerminalRenderable::new(PLAYER_DISPLAY_CHAR, Layer::base())),
            Box::new(PlayerMoveBehaviour::new()),
            Box::new(PlayerCombatBehaviour::new()),
        ]),
    );
    game.add_entity(
        Entity::new("Player Life Displayer", Transform::default()),
        BehaviourList::from(vec![Box::new(PlayerLifeDisplayBehaviour::new())]),
    );

    game.add_entity(
        Entity::new_with_id("Enemy Spawner", Transform::default(), ENEMY_SPAWNER_ID),
        BehaviourList::from(vec![Box::new(EnemySpawnerBehaviour::new())]),
    );

    game.add_entity(
        Entity::new("Star Spawner", Transform::default()),
        BehaviourList::from(vec![Box::new(StarSpawnerBehaviour::new())]),
    );

    game.start(&mut TerminalRenderer::new(TerminalRendererConfig {
        screen_resolution: Dimensions2d::new(SCREEN_HEIGHT as u64, SCREEN_WIDTH as u64),
        include_screen_outline: true,
    }));
}
