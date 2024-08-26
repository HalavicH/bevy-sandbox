use bevy::prelude::*;
use crate::components::Velocity;
use crate::plugins::game::spaceship::components::SpaceshipBundle;
use crate::plugins::game::spaceship::SpaceshipPlugin;


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        SpaceshipBundle {
            velocity: Velocity {
                vec3: STARTING_VELOCITY,
            },
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

pub fn update_position(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.vec3 * time.delta_seconds();
    }
}

pub fn print_position(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("Spaceship at: {:?}", transform.translation);
    }
}