use thomas::core::{BehaviourList, Entity, Layer, TerminalRenderable, Transform};

use crate::{EnemyBehaviour, EnemyCombatBehaviour};

pub fn make_enemy(transform: Transform) -> (Entity, BehaviourList) {
    let entity = Entity::new("Enemy", transform);
    let behaviours = BehaviourList::from(vec![
        Box::new(TerminalRenderable::new('@', Layer::base())),
        Box::new(EnemyCombatBehaviour::new()),
        Box::new(EnemyBehaviour::new()),
    ]);

    (entity, behaviours)
}