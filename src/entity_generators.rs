use thomas::core::{BehaviourList, Coords, Entity, Layer, TerminalRenderable, Transform};

use crate::{
    behaviours::{BulletBehaviour, EnemyCombatBehaviour, TextBehaviour, StarBehaviour},
    constants::{BACKGROUND_LAYER, BULLET_LAYER},
    BulletType,
};

pub fn make_bullet(
    transform: Transform,
    bullet_type: BulletType,
    travel_direction: Coords,
) -> (Entity, BehaviourList) {
    let entity = Entity::new("bullet", transform);

    let display = match bullet_type {
        BulletType::Player => '.',
        BulletType::Enemy => '"',
    };

    let behaviours = BehaviourList::from(vec![
        Box::new(TerminalRenderable::new(display, Layer::new(BULLET_LAYER))),
        Box::new(BulletBehaviour::new(bullet_type, travel_direction)),
    ]);

    (entity, behaviours)
}

pub fn make_enemy(transform: Transform) -> (Entity, BehaviourList) {
    let entity = Entity::new("Enemy", transform);
    let behaviours = BehaviourList::from(vec![
        Box::new(TerminalRenderable::new('@', Layer::base())),
        Box::new(EnemyCombatBehaviour::new()),
    ]);

    (entity, behaviours)
}

pub fn make_text(transform: Transform, id: &str, text: &str) -> (Entity, BehaviourList) {
    let entity = Entity::new_with_id("Text", transform, id);
    let behaviours = BehaviourList::from(vec![Box::new(TextBehaviour::new(text))]);

    (entity, behaviours)
}

pub fn make_star(transform: Transform) -> (Entity, BehaviourList) {
    let entity = Entity::new("Star", transform);
    let behaviours = BehaviourList::from(vec![
        Box::new(StarBehaviour::new()),
        Box::new(TerminalRenderable::new('*', Layer::new(BACKGROUND_LAYER))),
    ]);

    (entity, behaviours)
}
