use std::collections::HashMap;

use thomas::{
    GameCommand, IntCoords2d, Keycode, Layer, Query, QueryResultList, System, SystemExtraArgs,
    SystemsGenerator, TerminalCollider, TerminalRenderer, TerminalTransform, Timer, EVENT_INIT,
    EVENT_UPDATE,
};

use crate::{
    make_bullet, BulletType, Combat, Player, PlayerMovement, Scorekeeper, PLAYER_COLLISION_LAYER,
    PLAYER_DISPLAY_CHAR, PLAYER_STARTING_LIVES, SCREEN_WIDTH, UI_Y_START_POSITION,
};

const MOVE_WAIT_TIME_MILLIS: u128 = 100;
const SHOOT_WAIT_TIME_MILLIS: u128 = 100;

type MovementToDirectionMap = HashMap<Keycode, IntCoords2d>;

pub struct PlayerSystemsGenerator {}
impl SystemsGenerator for PlayerSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (
                EVENT_INIT,
                System::new(vec![], |_, util| {
                    util.commands().issue(GameCommand::AddEntity(vec![
                        Box::new(Player {
                            lives: PLAYER_STARTING_LIVES,
                        }),
                        Box::new(PlayerMovement {
                            move_timer: Timer::start_new(),
                        }),
                        Box::new(Combat {
                            shoot_timer: Timer::start_new(),
                        }),
                        Box::new(TerminalTransform {
                            coords: IntCoords2d::new(
                                SCREEN_WIDTH as i64 / 2,
                                UI_Y_START_POSITION as i64,
                            ),
                        }),
                        Box::new(TerminalRenderer {
                            display: PLAYER_DISPLAY_CHAR,
                            layer: Layer::base(),
                        }),
                        Box::new(TerminalCollider {
                            is_active: true,
                            layer: PLAYER_COLLISION_LAYER,
                        }),
                    ]));

                    util.commands()
                        .issue(GameCommand::AddEntity(vec![Box::new(Scorekeeper {
                            score: 0,
                            // TODO: Probably load from FS.
                            high_score: 300,
                        })]))
                }),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new()
                        .has::<PlayerMovement>()
                        .has::<TerminalTransform>()],
                    movement,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new()
                        .has::<Player>()
                        .has::<TerminalTransform>()
                        .has::<Combat>()],
                    combat,
                ),
            ),
        ]
    }
}

fn movement(results: Vec<QueryResultList>, util: &SystemExtraArgs) {
    if let [player_query, ..] = &results[..] {
        for player_result in player_query {
            let mut movement = player_result.components().get_mut::<PlayerMovement>();
            let mut transform = player_result.components().get_mut::<TerminalTransform>();

            let movement_input_to_direction: MovementToDirectionMap = HashMap::from([
                (Keycode::A, IntCoords2d::left()),
                (Keycode::D, IntCoords2d::right()),
            ]);

            if movement.move_timer.elapsed_millis() >= MOVE_WAIT_TIME_MILLIS {
                if let Some(movement_direction) =
                    get_movement_direction(util, &movement_input_to_direction)
                {
                    transform.coords += *movement_direction;

                    movement.move_timer.restart();
                }
            }
        }
    }
}

fn get_movement_direction<'a>(
    util: &SystemExtraArgs,
    movement_input_to_direction: &'a MovementToDirectionMap,
) -> Option<&'a IntCoords2d> {
    if let Some(pressed_key) = get_pressed_movement_key(util, movement_input_to_direction) {
        return movement_input_to_direction.get(pressed_key);
    }

    None
}

fn get_pressed_movement_key<'a>(
    util: &SystemExtraArgs,
    movement_input_to_direction: &'a MovementToDirectionMap,
) -> Option<&'a Keycode> {
    movement_input_to_direction
        .keys()
        .find(|key| util.input().is_key_pressed(key))
}

fn combat(results: Vec<QueryResultList>, util: &SystemExtraArgs) {
    if let [player_query, ..] = &results[..] {
        for player_result in player_query {
            let transform = player_result.components().get::<TerminalTransform>();
            let mut combat = player_result.components().get_mut::<Combat>();

            if util.input().is_key_pressed(&Keycode::Space)
                && combat.shoot_timer.elapsed_millis() >= SHOOT_WAIT_TIME_MILLIS
            {
                util.commands().issue(GameCommand::AddEntity(make_bullet(
                    transform.coords + IntCoords2d::down(),
                    IntCoords2d::down(),
                    BulletType::Player,
                )));

                combat.shoot_timer.restart();
            }
        }
    }
}
