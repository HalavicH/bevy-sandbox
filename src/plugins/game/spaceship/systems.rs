use crate::plugins::game::assets::GameAssets;
use crate::plugins::game::movement::components::{Acceleration, Velocity};
use crate::plugins::game::movement::MovingObjectBundle;
use crate::plugins::game::spaceship::components::{Projectile, Spaceship};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::thread;
use std::time::Duration;
use bevy::asset::LoadState;
use bevy::render::mesh::VertexAttributeValues;
use blenvy::{BluePrintBundle, BlueprintInfo, SpawnBlueprint};
use crate::plugins::game::collision::{Colliders, Size};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

const SPACESHIP_SPEED_SCALAR: f32 = 25.0;
const SPACESHIP_ROTATION_SCALAR: f32 = 2.5;
const SPACESHIP_ROLL_SCALAR: f32 = 5.0;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
struct LazyLoad;

fn calculate_bounding_box(mesh: &Mesh) -> (Vec3, Vec3) {
    let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(VertexAttributeValues::Float32x3(positions)) => positions,
        _ => panic!("Mesh does not have positions"),
    };

    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);

    for &position in positions.iter() {
        let position = Vec3::from(position);
        min = min.min(position);
        max = max.max(position);
    }

    (min, max)
}

pub fn spawn_spaceship_from_blueprint_example(mut commands: Commands) {
    commands
        .spawn((
            BlueprintInfo {
                name: "Spaceship".into(),
                path: "blueprints/Spaceship.glb".into(),
            },
            SpawnBlueprint::default(),
            Velocity {
                value: STARTING_VELOCITY,
            },
            Acceleration::default(),
            Colliders::new(Size { width: 5.0, height: 5.0 }),
            Transform::from_translation(STARTING_TRANSLATION),
        ))
        .insert(Spaceship);
}

// pub fn spawn_spaceship(
//     mut commands: Commands,
//     game_assets: Res<GameAssets>,
// ) {
//     commands
//         .spawn(MovingObjectBundle {
//             velocity: Velocity {
//                 value: STARTING_VELOCITY,
//             },
//             acceleration: Default::default(),
//             model: SceneBundle {
//                 scene: game_assets.get_spaceship(),
//                 transform: Transform::from_translation(STARTING_TRANSLATION),
//                 global_transform: Default::default(),
//                 visibility: Default::default(),
//                 inherited_visibility: Default::default(),
//                 view_visibility: Default::default(),
//             },
//             colliders: Colliders::new(Size { width: 5.0, height: 5.0 }),
//         })
//         .insert(Spaceship);
// }

pub fn move_spaceship(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Spaceship>>,
) {
    let Ok((mut velocity, mut transform)) = query.get_single_mut() else {
        debug!("Spaceship not found or multiple spaceships found");
        return;
    };

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if input.just_pressed(KeyCode::KeyW) || input.pressed(KeyCode::KeyW) {
        movement += SPACESHIP_SPEED_SCALAR;
    }
    if input.just_pressed(KeyCode::KeyS) || input.pressed(KeyCode::KeyS) {
        movement -= SPACESHIP_SPEED_SCALAR;
    }

    if input.just_pressed(KeyCode::KeyA) || input.pressed(KeyCode::KeyA) {
        rotation += SPACESHIP_ROTATION_SCALAR * time.delta_seconds();
    }
    if input.just_pressed(KeyCode::KeyD) || input.pressed(KeyCode::KeyD) {
        rotation -= SPACESHIP_ROTATION_SCALAR * time.delta_seconds();
    }

    if input.just_pressed(KeyCode::KeyQ) || input.pressed(KeyCode::KeyQ) {
        roll += SPACESHIP_ROLL_SCALAR * time.delta_seconds();
    }
    if input.just_pressed(KeyCode::KeyE) || input.pressed(KeyCode::KeyE) {
        roll -= SPACESHIP_ROLL_SCALAR * time.delta_seconds();
    }

    // We negate the forward direction because bevy inverts the z-axis
    let forward_direction = -transform.forward();

    velocity.value = forward_direction * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

#[derive(Resource)]
pub struct ProjectileTimer {
    pub value: Timer,
}

impl Default for ProjectileTimer {
    fn default() -> Self {
        Self {
            value: Timer::from_seconds(0.070, TimerMode::Repeating),
        }
    }
}

pub fn fire_projectile(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<&Transform, With<Spaceship>>,
    input: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<ProjectileTimer>,
    time: Res<Time>,
) {
    let Ok(spaceship_transform) = query.get_single() else {
        debug!("Spaceship not found or multiple spaceships found");
        return;
    };

    const PROJ_SPEED: f32 = 50.0;
    const NOSE_OFFSET: f32 = 8.0;
    const PROJ_SCALE_FACTOR: Vec3 = Vec3::splat(0.5);

    timer.value.tick(time.delta());
    if !input.pressed(KeyCode::Space) {
        return;
    }

    if !timer.value.finished() {
        return;
    }

    // We negate the forward direction because bevy inverts the z-axis
    let spaceship_forward_direction = -spaceship_transform.forward();
    let spaceship_nose_vec = spaceship_forward_direction * NOSE_OFFSET;
    let mut transform =
        Transform::from_translation(spaceship_transform.translation + spaceship_nose_vec);
    transform.scale = PROJ_SCALE_FACTOR;

    // Rotate the projectile 90 degrees around the X-axis
    transform.rotation = spaceship_transform.rotation * Quat::from_rotation_x(FRAC_PI_2);

    let handle = game_assets.get_projectile();
    // let model_size = game_assets.get_model_size(handle);
    let colliders = Colliders::new(Size { width: 1.0, height: 3.0 });
    commands
        .spawn(MovingObjectBundle {
            velocity: Velocity::new(spaceship_forward_direction * PROJ_SPEED),
            acceleration: Default::default(),
            model: SceneBundle {
                scene: handle,
                transform,
                ..SceneBundle::default()
            },
            colliders,
        })
        .insert(Projectile);
}
