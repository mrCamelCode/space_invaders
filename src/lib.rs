mod constants;
use std::{thread, time::Duration};

pub use constants::*;

mod systems;
pub use systems::*;

mod components;
pub use components::*;

mod prefabs;
pub use prefabs::*;

use thomas::{
    Dimensions2d, EngineAnalysisSystemsGenerator, EngineStats, Game, GameOptions, Identity,
    IntCoords2d, Query, Renderer, System, TerminalCollisionsSystemsGenerator,
    TerminalRendererOptions, TerminalUiRendererSystemsGenerator, Text, Time,
};

pub fn run() {
    Game::new(GameOptions {
        max_frame_rate: 60,
        press_escape_to_quit: false,
    })
    .add_systems_from_generator(EngineAnalysisSystemsGenerator::new())
    .add_systems_from_generator(TerminalCollisionsSystemsGenerator::new())
    .add_systems_from_generator(TerminalUiRendererSystemsGenerator::new())
    .add_systems_from_generator(HudSystemsGenerator {})
    .add_systems_from_generator(PlayerSystemsGenerator {})
    .add_systems_from_generator(BulletSystemsGenerator {})
    .add_systems_from_generator(EnemySystemsGenerator {})
    .add_init_system(System::new(vec![], |_, commands| {
        commands
            .borrow_mut()
            .issue(thomas::GameCommand::AddEntity(vec![
                Box::new(Text {
                    anchor: thomas::UiAnchor::TopLeft,
                    value: String::from(""),
                    justification: thomas::Alignment::Left,
                    offset: IntCoords2d::zero(),
                }),
                Box::new(Identity {
                    id: String::from("frame-counter-tag"),
                    name: String::from(""),
                }),
            ]))
    }))
    .add_update_system(System::new(
        vec![
            Query::new().has::<EngineStats>(),
            Query::new()
                .has_where::<Identity>(|id| id.id == "frame-counter-tag")
                .has::<Text>(),
            Query::new().has::<Time>()
        ],
        |results, _| {
            if let [engine_stats_results, fps_tag_results, time_results, ..] = &results[..] {
                let engine_stats = engine_stats_results.get_only::<EngineStats>();
                let mut fps_tag = fps_tag_results.get_only_mut::<Text>();
                let time = time_results.get_only::<Time>();

                fps_tag.value = format!("FPS: {}", engine_stats.fps);
                // fps_tag.value = format!("delta time: {}", time.delta_time());
            }
        },
    ))
    .start(Renderer::Terminal(TerminalRendererOptions {
        include_default_camera: true,
        screen_resolution: Dimensions2d::new(SCREEN_HEIGHT, SCREEN_WIDTH),
        include_screen_outline: true,
    }));
}
