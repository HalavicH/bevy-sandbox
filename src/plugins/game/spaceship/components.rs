use bevy::prelude::*;
use crate::plugins::game::movement::components::Velocity;

#[derive(Component, Default, Clone)]
pub struct Spaceship;

#[derive(Component, Default, Clone)]
pub struct Projectile;