use bevy::prelude::*;
use crate::components::{Acceleration, Velocity};

pub struct MovementPlugin;

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            // update_acceleration_from_keys,
            update_position,
            update_velocity
        ));
    }
}

pub fn update_acceleration_from_keys(
    mut query: Query<&mut Acceleration>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const ACCELERATION_RATE: f32 = 1.0;
    for mut acceleration in query.iter_mut() {
        let mut value = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) {
            value.z += ACCELERATION_RATE;
        }
        if input.pressed(KeyCode::KeyS) {
            value.z -= ACCELERATION_RATE;
        }
        if input.pressed(KeyCode::KeyA) {
            value.x -= ACCELERATION_RATE;
        }
        if input.pressed(KeyCode::KeyD) {
            value.x += ACCELERATION_RATE;
        }
        acceleration.value = value.normalize();
    }
}

pub fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

pub fn update_position(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
