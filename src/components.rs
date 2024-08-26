use bevy::math::Vec3;
use bevy::prelude::{Component};

#[derive(Component, Default, Clone)]
pub struct Velocity {
    pub vec3: Vec3
}

