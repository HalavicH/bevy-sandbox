use bevy::prelude::*;
use crate::plugins::game::spaceship::SpaceshipPlugin;

mod spaceship;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SpaceshipPlugin);
    }
}