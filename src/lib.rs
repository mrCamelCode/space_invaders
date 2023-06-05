mod constants;
pub use constants::*;

mod systems;
pub use systems::*;

mod components;
pub use components::*;

mod prefabs;
pub use prefabs::*;

use thomas::{Dimensions2d, Game, GameOptions, Renderer, TerminalRendererOptions};

pub fn run() {
    Game::new(GameOptions {
        max_frame_rate: 60,
        press_escape_to_quit: false,
    })
    .add_systems_from_generator(PlayerSystemsGenerator {})
    .add_systems_from_generator(BulletSystemsGenerator {})
    .start(Renderer::Terminal(TerminalRendererOptions {
        include_default_camera: true,
        screen_resolution: Dimensions2d::new(SCREEN_HEIGHT, SCREEN_WIDTH),
        include_screen_outline: true,
    }));
}
