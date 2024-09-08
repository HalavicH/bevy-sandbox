use bevy::prelude::*;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Spaceship;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Projectile;

#[derive(Resource, Debug)]
pub struct PlayerStats {
    pub score: i32,
    pub health: i32,
    pub ammo_left: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            score: 0,
            health: 100,
            ammo_left: 100,
        }
    }
}
