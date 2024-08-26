use bevy::math::Vec3;
use bevy::prelude::{Component};

#[derive(Component, Default, Clone)]
pub struct Velocity {
    pub value: Vec3
}

#[derive(Component, Default, Clone)]
pub struct Acceleration {
    pub value: Vec3
}