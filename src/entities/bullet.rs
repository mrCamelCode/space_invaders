use thomas::core::{BehaviourList, Coords, Entity, Layer, TerminalRenderable, Transform};

use crate::{behaviours::BulletType, constants::BULLET_LAYER, BulletBehaviour};

pub fn make_bullet(
    transform: Transform,
    bullet_type: BulletType,
    travel_direction: Coords,
) -> (Entity, BehaviourList) {
    let entity = Entity::new("bullet", transform);

    let display = match bullet_type {
        BulletType::Player => '.',
        BulletType::Enemy => 'o',
    };

    let behaviours = BehaviourList::from(vec![
        Box::new(TerminalRenderable::new(display, Layer::new(BULLET_LAYER))),
        Box::new(BulletBehaviour::new(bullet_type, travel_direction)),
    ]);

    (entity, behaviours)
}
