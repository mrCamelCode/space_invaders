use thomas::core::{BehaviourList, Entity, Transform};

use crate::TextBehaviour;

pub fn make_text(transform: Transform, id: &str, text: &str) -> (Entity, BehaviourList) {
    let entity = Entity::new_with_id("Text", transform, id);
    let behaviours = BehaviourList::from(vec![Box::new(TextBehaviour::new(text))]);

    (entity, behaviours)
}
