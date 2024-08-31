use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component, Default, Clone)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Default, Clone)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Default, Clone)]
pub struct SpinVelocity {
    pub value: Vec3,
}

impl SpinVelocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}
