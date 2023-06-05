use thomas::{
    Component, IntCoords2d, Layer, TerminalCollider, TerminalRenderer, TerminalTransform, Timer,
};

use crate::{
    Bullet, ENEMY_BULLET_COLLISION_LAYER, ENEMY_BULLET_DISPLAY_CHAR, PLAYER_BULLET_COLLISION_LAYER,
    PLAYER_BULLET_DISPLAY_CHAR,
};

#[derive(PartialEq, Eq)]
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
