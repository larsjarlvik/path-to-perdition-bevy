use crate::components;
use bevy::prelude::*;

pub(crate) fn movement(time: Res<Time>, mut query: Query<(&mut Transform, &components::Movement)>) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation.x += movement.max_velocity * time.delta_seconds();
    }
}
