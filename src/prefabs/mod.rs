use thomas::{
    Component, IntCoords2d, Layer, TerminalCollider, TerminalRenderer, TerminalTransform, Timer,
};

use crate::{
    Bullet, Enemy, ENEMY_BULLET_COLLISION_LAYER, ENEMY_BULLET_DISPLAY_CHAR, ENEMY_COLLISION_LAYER,
    ENEMY_DISPLAY_CHAR, PLAYER_BULLET_COLLISION_LAYER, PLAYER_BULLET_DISPLAY_CHAR,
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
            layer: Layer::base(),
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
        Box::new(Enemy {}),
        Box::new(TerminalTransform { coords }),
        Box::new(TerminalRenderer {
            display: ENEMY_DISPLAY_CHAR,
            layer: Layer::base(),
        }),
        Box::new(TerminalCollider {
            is_active: true,
            layer: ENEMY_COLLISION_LAYER,
        }),
    ]
}
