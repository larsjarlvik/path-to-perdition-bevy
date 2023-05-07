use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Movement {
    pub max_velocity: f32,
    pub velocity: f32,
    pub target_velocity: f32,
    pub direction: f32,
    pub to: Vec3,
}

impl Movement {
    pub fn new(max_velocity: f32) -> Self {
        Self {
            max_velocity,
            velocity: 0.0,
            target_velocity: 0.0,
            direction: 0.0,
            to: Vec3::ZERO,
        }
    }

    pub fn towards(&mut self, direction: Vec3) {
        self.direction = direction.x.atan2(direction.z);
        self.velocity = self.velocity.clamp(-self.max_velocity, self.max_velocity);
    }
}
