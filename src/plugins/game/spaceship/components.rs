use bevy::prelude::*;
use crate::components::Velocity;

#[derive(Bundle)]
pub struct SpaceshipBundle {
    pub velocity: Velocity,
    pub model: SceneBundle,
}