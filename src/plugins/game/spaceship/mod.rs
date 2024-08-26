use bevy::prelude::*;
use crate::plugins::game::spaceship::systems::*;

pub mod components;
mod systems;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}