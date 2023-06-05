use thomas::Layer;

pub const SCREEN_WIDTH: u64 = 50;
pub const SCREEN_HEIGHT: u64 = 20;

pub const UI_Y_START_POSITION: u64 = 17;

pub const PLAYER_DISPLAY_CHAR: char = 'A';
pub const PLAYER_COLLISION_LAYER: Layer = Layer(1);
pub const PLAYER_BULLET_DISPLAY_CHAR: char = '"';
pub const PLAYER_BULLET_COLLISION_LAYER: Layer = Layer(3);
pub const PLAYER_STARTING_LIVES: u8 = 3;

pub const BACKGROUND_LAYER: i32 = -10;
pub const BULLET_LAYER: i32 = -1;

pub const ENEMY_DISPLAY_CHAR: char = '@';
pub const ENEMY_COLLISION_LAYER: Layer = Layer(2);
pub const ENEMY_BULLET_DISPLAY_CHAR: char = 'o';
pub const ENEMY_BULLET_COLLISION_LAYER: Layer = Layer(4);
pub const ENEMY_POINT_VALUE: u64 = 100;
