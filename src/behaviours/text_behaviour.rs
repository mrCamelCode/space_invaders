use thomas::{
    core::{
        Behaviour, BehaviourList, BehaviourUtils, Coords, CustomBehaviour, Entity, GameCommand,
        Layer, Message, TerminalRenderable, Transform, World,
    },
    Behaviour,
};

use crate::{ChangeTextPayload, MSG_CHANGE_TEXT, UI_LAYER};

#[derive(Behaviour, Clone)]
pub struct TextBehaviour {
    char_entity_ids: Vec<String>,
    text: String,
    should_generate_text: bool,
}
impl TextBehaviour {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            char_entity_ids: vec![],
            should_generate_text: false,
        }
    }

    fn delete_existing_character_entities(&mut self, utils: &mut BehaviourUtils) {
        for existing_char_entity_id in self.char_entity_ids.iter() {
            utils
                .commands
                .issue(GameCommand::DestroyEntity(existing_char_entity_id.clone()));
        }

        self.char_entity_ids.clear();
    }

    fn generate_text(&mut self, utils: &mut BehaviourUtils) {
        self.delete_existing_character_entities(utils);

        let BehaviourUtils {
            entity, commands, ..
        } = utils;

        let start_pos = entity.transform().coords();

        for (i, character) in self.text.char_indices() {
            let (char_entity, behaviours) = make_char_entity(
                Transform::new(Coords::new(start_pos.x() + i as f64, start_pos.y(), 0.0)),
                entity.id(),
                character,
                i,
            );

            self.char_entity_ids.push(char_entity.id().to_string());

            commands.issue(GameCommand::AddEntity {
                entity: char_entity,
                behaviours,
            });
        }
    }
}
impl CustomBehaviour for TextBehaviour {
    fn init(&mut self, utils: &mut BehaviourUtils) {
        self.generate_text(utils);
    }

    fn update(&mut self, utils: &mut thomas::core::BehaviourUtils) {
        if self.should_generate_text {
            self.generate_text(utils);

            self.should_generate_text = false;
        }
    }

    fn on_message(&mut self, message: &Message<Box<dyn Any>>) {
        match message.typ.as_str() {
            MSG_CHANGE_TEXT => {
                if let Some(payload) = Message::<ChangeTextPayload>::get_payload(message) {
                    if payload.new_text != self.text {
                        self.text = payload.new_text.clone();

                        self.should_generate_text = true;
                    }
                }
            }
            _ => {}
        }
    }
}

fn get_char_entity_id(text_entity_id: &str, character: char, index: usize) -> String {
    format!("{text_entity_id}-{character}-{index}")
}

fn make_char_entity(
    transform: Transform,
    text_entity_id: &str,
    character: char,
    index: usize,
) -> (Entity, BehaviourList) {
    let entity = Entity::new_with_id(
        "text char",
        transform,
        &get_char_entity_id(text_entity_id, character, index),
    );

    let behaviours = BehaviourList::from(vec![Box::new(TerminalRenderable::new(
        character,
        Layer::new(UI_LAYER),
    ))]);

    (entity, behaviours)
}
