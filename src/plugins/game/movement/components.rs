use bevy::prelude::*;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct SpinVelocity {
    pub value: Vec3,
}

impl SpinVelocity {
    #[allow(dead_code)]
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}
