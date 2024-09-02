use crate::plugins::game::spaceship::systems::*;
use bevy::prelude::*;
use crate::plugins::game::assets::load_assets;

pub mod components;
mod systems;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileTimer>()
            .add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, (move_spaceship, fire_projectile));
            // .add_systems(Update, (load_assets));
    }
}
