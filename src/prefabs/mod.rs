use rand::{thread_rng, Rng};
use thomas::{
    Component, IntCoords2d, Layer, TerminalCollider, TerminalRenderer, TerminalTransform, Timer, Rgb,
};

use crate::{
    Bullet, Enemy, Star, ENEMY_BULLET_COLLISION_LAYER, ENEMY_BULLET_DISPLAY_CHAR,
    ENEMY_COLLISION_LAYER, ENEMY_DISPLAY_CHAR, PLAYER_BULLET_COLLISION_LAYER,
    PLAYER_BULLET_DISPLAY_CHAR, SCREEN_WIDTH, ENEMY_COLOR, PLAYER_COLOR,
};

pub enum BulletType {
    Player,
    Enemy,
}

pub fn make_bullet(
    start_pos: IntCoords2d,
    direction: IntCoords2d,
    bullet_type: BulletType,
) -> Vec<Box<dyn Component>> {
    vec![
        Box::new(Bullet {
            direction,
            move_timer: Timer::start_new(),
        }),
        Box::new(TerminalTransform { coords: start_pos }),
        Box::new(TerminalRenderer {
            display: match bullet_type {
                BulletType::Player => PLAYER_BULLET_DISPLAY_CHAR,
                BulletType::Enemy => ENEMY_BULLET_DISPLAY_CHAR,
            },
            layer: Layer::below(&Layer::base()),
            foreground_color: match bullet_type {
                BulletType::Player => Some(PLAYER_COLOR),
                BulletType::Enemy => Some(ENEMY_COLOR),
            },
            background_color: None,
        }),
        Box::new(TerminalCollider {
            is_active: true,
            layer: match bullet_type {
                BulletType::Player => PLAYER_BULLET_COLLISION_LAYER,
                BulletType::Enemy => ENEMY_BULLET_COLLISION_LAYER,
            },
        }),
    ]
}

pub fn make_enemy(coords: IntCoords2d) -> Vec<Box<dyn Component>> {
    vec![
        Box::new(Enemy {
            shoot_timer: Timer::start_new(),
        }),
        Box::new(TerminalTransform { coords }),
        Box::new(TerminalRenderer {
            display: ENEMY_DISPLAY_CHAR,
            layer: Layer::base(),
            foreground_color: Some(ENEMY_COLOR),
            background_color: None,
        }),
        Box::new(TerminalCollider {
            is_active: true,
            layer: ENEMY_COLLISION_LAYER,
        }),
    ]
}

pub fn make_star(coords: Option<IntCoords2d>) -> Vec<Box<dyn Component>> {
    vec![
        Box::new(Star {
            move_timer: Timer::start_new(),
            move_wait_time: thread_rng().gen_range(200..=600),
        }),
        Box::new(TerminalTransform {
            coords: if let Some(c) = coords {
                c
            } else {
                IntCoords2d::new(thread_rng().gen_range(0..SCREEN_WIDTH) as i64, -1)
            },
        }),
        Box::new(TerminalRenderer {
            display: '*',
            layer: Layer::above(&Layer::furthest_background()),
            foreground_color: Some(Rgb::white()),
            background_color: None,
        }),
    ]
}
