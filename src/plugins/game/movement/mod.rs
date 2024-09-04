use std::f32::consts::TAU;
use crate::plugins::game::movement::components::{Acceleration, SpinVelocity, Velocity};
use bevy::prelude::*;
use crate::plugins::game::collision::Colliders;

pub mod components;

pub struct MovementPlugin;

#[derive(Bundle, Default)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
    pub colliders: Colliders,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_velocity,
                update_position,
                update_rotation,
            ),
        );
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

pub fn update_rotation(time: Res<Time>, mut query: Query<(&SpinVelocity, &mut Transform)>) {
    for (spin_velocity, mut transform) in query.iter_mut() {
        transform.rotate_local_x(spin_velocity.value.x * time.delta_seconds() * TAU);
        transform.rotate_local_y(spin_velocity.value.y * time.delta_seconds() * TAU);
        transform.rotate_local_z(spin_velocity.value.z * time.delta_seconds() * TAU);
    }
}
