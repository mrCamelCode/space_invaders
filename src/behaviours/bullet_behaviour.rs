use thomas::core::{
    Behaviour, BehaviourUtils, CustomBehaviour, GameCommand, Layer, Message, Renderable,
    TerminalRenderable, Timer,
};
use thomas::get_behaviour_name;
use thomas::{core::Coords, Behaviour};

use crate::{BulletHitPayload, MSG_BULLET_HIT, SCREEN_HEIGHT};

const MOVE_WAIT_TIME_MILLIS: u128 = 50;

#[derive(PartialEq, Clone)]
pub enum BulletType {
    Player,
    Enemy,
}

#[derive(Behaviour, Clone)]
pub struct BulletBehaviour {
    typ: BulletType,
    travel_direction: Coords,
    move_timer: Timer,
}
impl BulletBehaviour {
    pub fn new(typ: BulletType, travel_direction: Coords) -> Self {
        Self {
            typ,
            travel_direction,
            move_timer: Timer::new(),
        }
    }

    fn handle_collisions(&self, utils: &mut BehaviourUtils) {
        let BehaviourUtils {
            commands,
            world,
            entity,
            ..
        } = utils;

        for (overlapping_entity, behaviours) in world.get_overlapping_entities(entity.id()) {
            if let Some(terminal_renderable_behaviour) = behaviours
                .get_behaviour::<TerminalRenderable>(get_behaviour_name!(TerminalRenderable))
            {
                if terminal_renderable_behaviour
                    .layer()
                    .is_with(&Layer::base())
                {
                    commands.issue(GameCommand::SendMessage {
                        entity_id: overlapping_entity.id().to_string(),
                        message: Message::new(
                            MSG_BULLET_HIT,
                            Box::new(BulletHitPayload {
                                bullet_type: self.typ.clone(),
                            }),
                        ),
                    });

                    entity.destroy();
                }
            }
        }
    }
}
impl CustomBehaviour for BulletBehaviour {
    fn init(&mut self, _: &mut BehaviourUtils) {
        self.move_timer.start();
    }

    fn update(&mut self, utils: &mut BehaviourUtils) {
        let BehaviourUtils { entity, .. } = utils;

        if self.move_timer.elapsed_millis() >= MOVE_WAIT_TIME_MILLIS {
            entity.transform_mut().move_by(&self.travel_direction);
            self.move_timer.restart();
        }

        if entity.transform().coords().y() > SCREEN_HEIGHT as f64
            || entity.transform().coords().y() < 0.0
        {
            entity.destroy();
        }

        self.handle_collisions(utils);
    }
}
