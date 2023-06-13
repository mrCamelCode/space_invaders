use std::{cell::Ref, collections::HashMap};

use thomas::{
    GameCommand, GameCommandsArg, Input, IntCoords2d, Keycode, Layer, Query, QueryResultList,
    System, SystemsGenerator, TerminalCollider, TerminalRenderer, TerminalTransform, Timer,
    EVENT_INIT, EVENT_UPDATE
};

use crate::{
    make_bullet, Bullet, BulletType, Combat, Enemy, Player, PlayerMovement, Scorekeeper,
    PLAYER_COLLISION_LAYER, PLAYER_DISPLAY_CHAR, PLAYER_STARTING_LIVES, SCREEN_WIDTH,
    UI_Y_START_POSITION, PLAYER_COLOR,
};

const MOVE_WAIT_TIME_MILLIS: u128 = 50;
const SHOOT_WAIT_TIME_MILLIS: u128 = 100;

type MovementToDirectionMap = HashMap<Keycode, IntCoords2d>;

pub struct PlayerSystemsGenerator {}
impl SystemsGenerator for PlayerSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (
                EVENT_INIT,
                System::new(vec![], |_, commands| {
                    commands.borrow_mut().issue(GameCommand::AddEntity(vec![
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
                            foreground_color: Some(PLAYER_COLOR),
                            background_color: None
                        }),
                        Box::new(TerminalCollider {
                            is_active: true,
                            layer: PLAYER_COLLISION_LAYER,
                        }),
                    ]));

                    commands
                        .borrow_mut()
                        .issue(GameCommand::AddEntity(vec![Box::new(Scorekeeper {
                            score: 0,
                            high_score: 10000,
                            level: 0,
                        })]))
                }),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has::<PlayerMovement>()
                            .has::<TerminalTransform>(),
                        Query::new().has::<Input>(),
                    ],
                    movement,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has::<Player>()
                            .has::<TerminalTransform>()
                            .has::<Combat>(),
                        Query::new()
                            .has_where::<Input>(|input| input.is_key_pressed(&Keycode::Space)),
                    ],
                    combat,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has::<Player>(),
                        Query::new().has::<Scorekeeper>(),
                        Query::new().has::<Enemy>(),
                        Query::new().has::<Bullet>(),
                    ],
                    handle_player_death,
                ),
            ),
        ]
    }
}

fn movement(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [player_results, input_results, ..] = &results[..] {
        let input = input_results.get_only::<Input>();

        let mut movement = player_results.get_only_mut::<PlayerMovement>();
        let mut transform = player_results.get_only_mut::<TerminalTransform>();

        let movement_input_to_direction: MovementToDirectionMap = HashMap::from([
            (Keycode::A, IntCoords2d::left()),
            (Keycode::D, IntCoords2d::right()),
        ]);

        if movement.move_timer.elapsed_millis() >= MOVE_WAIT_TIME_MILLIS {
            if let Some(movement_direction) =
                get_movement_direction(input, &movement_input_to_direction)
            {
                transform.coords += *movement_direction;

                transform.coords = IntCoords2d::new(
                    transform.coords.x().clamp(0, SCREEN_WIDTH as i64 - 1),
                    transform.coords.y(),
                );

                movement.move_timer.restart();
            }
        }
    }
}

fn get_movement_direction<'a>(
    input: Ref<Input>,
    movement_input_to_direction: &'a MovementToDirectionMap,
) -> Option<&'a IntCoords2d> {
    if let Some(pressed_key) = get_pressed_movement_key(input, movement_input_to_direction) {
        return movement_input_to_direction.get(pressed_key);
    }

    None
}

fn get_pressed_movement_key<'a>(
    input: Ref<Input>,
    movement_input_to_direction: &'a MovementToDirectionMap,
) -> Option<&'a Keycode> {
    movement_input_to_direction
        .keys()
        .find(|key| input.is_key_pressed(key))
}

fn combat(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [player_query, input_with_shoot_button_pressed_results, ..] = &results[..] {
        if input_with_shoot_button_pressed_results.len() > 0 {
            for player_result in player_query {
                let transform = player_result.components().get::<TerminalTransform>();
                let mut combat = player_result.components().get_mut::<Combat>();

                if combat.shoot_timer.elapsed_millis() >= SHOOT_WAIT_TIME_MILLIS {
                    commands
                        .borrow_mut()
                        .issue(GameCommand::AddEntity(make_bullet(
                            transform.coords + IntCoords2d::down(),
                            IntCoords2d::down(),
                            BulletType::Player,
                        )));

                    combat.shoot_timer.restart();
                }
            }
        }
    }
}

fn handle_player_death(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [player_results, scorekeeper_results, enemies_results, bullets_results, ..] =
        &results[..]
    {
        let mut player = player_results.get_only_mut::<Player>();
        let mut scorekeeper = scorekeeper_results.get_only_mut::<Scorekeeper>();

        if player.lives == 0 {
            scorekeeper.score = 0;
            scorekeeper.level = 0;

            player.lives = PLAYER_STARTING_LIVES;

            for enemy_result in enemies_results {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*enemy_result.entity()));
            }

            for bullet_result in bullets_results {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*bullet_result.entity()));
            }
        }
    }
}
