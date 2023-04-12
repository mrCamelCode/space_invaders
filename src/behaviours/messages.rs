use super::BulletType;

pub const MSG_BULLET_HIT: &str = "bullethit";
pub struct BulletHitPayload {
    pub bullet_type: BulletType,
}

pub const MSG_ENEMY_DIED: &str = "enemydied";
