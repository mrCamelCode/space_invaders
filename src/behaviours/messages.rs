use thomas::core::Coords;

use super::BulletType;

pub const MSG_BULLET_HIT: &str = "bullethit";
pub struct BulletHitPayload {
    pub bullet_type: BulletType,
}

pub const MSG_ENEMY_DIED: &str = "enemydied";
pub struct EnemyDiedPayload {
    pub enemy_id: String,
}

pub const MSG_PLAYER_KILLED_ENEMY: &str = "enemydiedbyplayer";

pub const MSG_RESET: &str = "reset";

pub const MSG_CHANGE_TEXT: &str = "updatetxt";
pub struct ChangeTextPayload {
    pub new_text: String,
}

pub const MSG_MOVE_ENEMY: &str = "moveenemy";
pub struct MoveEnemyPayload {
    pub displacement: Coords,
}
