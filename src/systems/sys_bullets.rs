use thomas::{
    GameCommand, GameCommandsArg, Query, QueryResultList, System, SystemsGenerator,
    TerminalCollision, TerminalTransform, EVENT_UPDATE,
};

use crate::{
    Bullet, Player, Scorekeeper, ENEMY_BULLET_COLLISION_LAYER, ENEMY_COLLISION_LAYER,
    ENEMY_POINT_VALUE, PLAYER_BULLET_COLLISION_LAYER, PLAYER_COLLISION_LAYER, SCREEN_HEIGHT,
};

const BULLET_MOVE_WAIT_TIME_MILLIS: u128 = 50;

pub struct BulletSystemsGenerator {}
impl SystemsGenerator for BulletSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has::<Bullet>().has::<TerminalTransform>()],
                    move_bullets,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<TerminalCollision>(|collision| {
                            let has_player_bullet = collision.bodies.iter().any(|(_, collider)| {
                                collider.layer == PLAYER_BULLET_COLLISION_LAYER
                            });
                            let has_enemy = collision
                                .bodies
                                .iter()
                                .any(|(_, collider)| collider.layer == ENEMY_COLLISION_LAYER);

                            has_player_bullet && has_enemy
                        }),
                        Query::new().has::<Scorekeeper>(),
                    ],
                    player_bullet_hits_enemy_collisions,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new().has_where::<TerminalCollision>(|collision| {
                            let has_enemy_bullet = collision.bodies.iter().any(|(_, collider)| {
                                collider.layer == ENEMY_BULLET_COLLISION_LAYER
                            });
                            let has_player = collision
                                .bodies
                                .iter()
                                .any(|(_, collider)| collider.layer == PLAYER_COLLISION_LAYER);

                            has_enemy_bullet && has_player
                        }),
                        Query::new().has::<Player>(),
                    ],
                    enemy_bullet_hits_player_collisions,
                ),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![Query::new().has_where::<TerminalCollision>(|collision| {
                        let are_bullets_colliding =
                            collision.bodies.iter().any(|(_, collider)| {
                                collider.layer == ENEMY_BULLET_COLLISION_LAYER
                            }) && collision.bodies.iter().any(|(_, collider)| {
                                collider.layer == PLAYER_BULLET_COLLISION_LAYER
                            });

                        !are_bullets_colliding && collision.bodies.iter().any(|(_, collider)| {
                            collider.layer == ENEMY_BULLET_COLLISION_LAYER
                                || collider.layer == PLAYER_BULLET_COLLISION_LAYER
                        })
                    })],
                    cleanup_bullets_on_collision,
                ),
            ),
        ]
    }
}

fn move_bullets(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [bullets_query, ..] = &results[..] {
        for bullet_result in bullets_query {
            let mut bullet = bullet_result.components().get_mut::<Bullet>();
            let mut transform = bullet_result.components().get_mut::<TerminalTransform>();

            if transform.coords.y() < 0 || transform.coords.y() > SCREEN_HEIGHT as i64 {
                commands
                    .borrow_mut()
                    .issue(GameCommand::DestroyEntity(*bullet_result.entity()));
            }

            if bullet.move_timer.elapsed_millis() >= BULLET_MOVE_WAIT_TIME_MILLIS {
                transform.coords += bullet.direction;

                bullet.move_timer.restart();
            }
        }
    }
}

fn player_bullet_hits_enemy_collisions(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [bullet_collision_results, scorekeeper_results, ..] = &results[..] {
        let mut scorekeeper = scorekeeper_results[0].components().get_mut::<Scorekeeper>();

        for bullet_collision in bullet_collision_results {
            let collision = bullet_collision.components().get::<TerminalCollision>();

            commands.borrow_mut().issue(GameCommand::DestroyEntity(
                collision
                    .bodies
                    .iter()
                    .find(|(_, collider)| collider.layer == ENEMY_COLLISION_LAYER)
                    .unwrap()
                    .0,
            ));

            scorekeeper.score += ENEMY_POINT_VALUE;
        }
    }
}

fn enemy_bullet_hits_player_collisions(results: Vec<QueryResultList>, _: GameCommandsArg) {
    if let [bullet_collision_results, player_results, ..] = &results[..] {
        let mut player = player_results[0].components().get_mut::<Player>();

        for _ in bullet_collision_results {
            if player.lives > 0 {
                player.lives -= 1;
            }
        }
    }
}

fn cleanup_bullets_on_collision(results: Vec<QueryResultList>, commands: GameCommandsArg) {
    if let [bullet_collision_results, ..] = &results[..] {
        for bullet_collision in bullet_collision_results {
            let collision = bullet_collision.components().get::<TerminalCollision>();

            commands.borrow_mut().issue(GameCommand::DestroyEntity(
                collision
                    .bodies
                    .iter()
                    .find(|(_, collider)| {
                        collider.layer == ENEMY_BULLET_COLLISION_LAYER
                            || collider.layer == PLAYER_BULLET_COLLISION_LAYER
                    })
                    .unwrap()
                    .0,
            ))
        }
    }
}
