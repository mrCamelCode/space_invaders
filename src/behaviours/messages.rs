use super::BulletType;

pub const MSG_BULLET_HIT: &str = "bullethit";
pub struct BulletHitPayload {
    pub bullet_type: BulletType,
}

pub const MSG_ENEMY_DIED: &str = "enemydied";

pub const MSG_RESET: &str = "reset";

pub const MSG_CHANGE_TEXT: &str = "updatetxt";
pub struct ChangeTextPayload {
    pub new_text: String,
}
