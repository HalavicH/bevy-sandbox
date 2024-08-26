use bevy::prelude::*;
use crate::components::Velocity;
use crate::plugins::game::movement::MovingObjectBundle;


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        MovingObjectBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            acceleration: Default::default(),
            model: SceneBundle {
                scene: asset_server.load("models/Spaceship.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                global_transform: Default::default(),
                visibility: Default::default(),
                inherited_visibility: Default::default(),
                view_visibility: Default::default(),
            }
        }
    );
}
