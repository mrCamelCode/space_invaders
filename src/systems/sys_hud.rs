use thomas::{
    Alignment, GameCommand, Identity, IntCoords2d, Query, System, SystemsGenerator, Text, UiAnchor,
    EVENT_INIT, EVENT_UPDATE,
};

use crate::{Player, Scorekeeper, PLAYER_DISPLAY_CHAR};

const LIVES_TAG_ID: &str = "lives-tag";
const SCORE_TAG_ID: &str = "score-tag";
const HIGH_SCORE_TAG_ID: &str = "high-score-tag";

pub struct HudSystemsGenerator {}
impl SystemsGenerator for HudSystemsGenerator {
    fn generate(&self) -> Vec<(&'static str, System)> {
        vec![
            (
                EVENT_INIT,
                System::new(vec![], |_, util| {
                    util.commands().issue(GameCommand::AddEntity(vec![
                        Box::new(Text {
                            anchor: UiAnchor::BottomLeft,
                            justification: Alignment::Left,
                            offset: IntCoords2d::zero(),
                            value: String::from(""),
                        }),
                        Box::new(Identity {
                            id: String::from(LIVES_TAG_ID),
                            name: String::from(""),
                        }),
                    ]));

                    util.commands().issue(GameCommand::AddEntity(vec![
                        Box::new(Text {
                            anchor: UiAnchor::BottomRight,
                            justification: Alignment::Right,
                            offset: IntCoords2d::down(),
                            value: String::from(""),
                        }),
                        Box::new(Identity {
                            id: String::from(SCORE_TAG_ID),
                            name: String::from(""),
                        }),
                    ]));

                    util.commands().issue(GameCommand::AddEntity(vec![
                        Box::new(Text {
                            anchor: UiAnchor::BottomRight,
                            justification: Alignment::Right,
                            offset: IntCoords2d::zero(),
                            value: String::from(""),
                        }),
                        Box::new(Identity {
                            id: String::from(HIGH_SCORE_TAG_ID),
                            name: String::from(""),
                        }),
                    ]));
                }),
            ),
            (
                EVENT_UPDATE,
                System::new(
                    vec![
                        Query::new()
                            .has_where::<Identity>(|id| id.id == LIVES_TAG_ID)
                            .has::<Text>(),
                        Query::new()
                            .has_where::<Identity>(|id| id.id == SCORE_TAG_ID)
                            .has::<Text>(),
                        Query::new()
                            .has_where::<Identity>(|id| id.id == HIGH_SCORE_TAG_ID)
                            .has::<Text>(),
                        Query::new().has::<Player>(),
                        Query::new().has::<Scorekeeper>(),
                    ],
                    |results, _| {
                        if let [lives_tag_results, score_tag_results, high_score_tag_results, player_results, scorekeeper_results, ..] =
                            &results[..]
                        {
                            let player = player_results[0].components().get::<Player>();
                            let scorekeeper =
                                scorekeeper_results[0].components().get::<Scorekeeper>();

                            let mut lives_tag = lives_tag_results[0].components().get_mut::<Text>();
                            let mut score_tag = score_tag_results[0].components().get_mut::<Text>();
                            let mut high_score_tag =
                                high_score_tag_results[0].components().get_mut::<Text>();

                            lives_tag.value = format!(
                                "Lives: {}",
                                (0..player.lives)
                                    .map(|_| PLAYER_DISPLAY_CHAR.to_string())
                                    .collect::<Vec<String>>()
                                    .join("")
                            );

                            score_tag.value = format!("Score: {}", scorekeeper.score);

                            high_score_tag.value = format!(
                                "Hi-score: {}",
                                u64::max(scorekeeper.high_score, scorekeeper.score)
                            );
                        }
                    },
                ),
            ),
        ]
    }
}
