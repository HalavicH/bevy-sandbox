use bevy::prelude::*;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Spaceship;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Projectile;
