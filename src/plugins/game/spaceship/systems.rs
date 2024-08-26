use bevy::prelude::*;
use crate::plugins::game::assets::GameAssets;
use crate::plugins::game::movement::components::Velocity;
use crate::plugins::game::movement::MovingObjectBundle;
use crate::plugins::game::spaceship::components::Spaceship;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

const SPACESHIP_SPEED_SCALAR: f32 = 25.0;
const SPACESHIP_ROTATION_SCALAR: f32 = 2.5;
const SPACESHIP_ROLL_SCALAR: f32 = 5.0;

pub fn spawn_spaceship(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(
        MovingObjectBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            acceleration: Default::default(),
            model: SceneBundle {
                scene: game_assets.get_spaceship(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                global_transform: Default::default(),
                visibility: Default::default(),
                inherited_visibility: Default::default(),
                view_visibility: Default::default(),
            }
        }
    ).insert(Spaceship);
}

pub fn move_spaceship(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Spaceship>>,
) {
    let (mut velocity, mut transform) = query.single_mut();

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if input.just_pressed(KeyCode::KeyW) || input.pressed(KeyCode::KeyW) {
        movement += SPACESHIP_SPEED_SCALAR;
    } else if input.just_pressed(KeyCode::KeyS) || input.pressed(KeyCode::KeyS) {
        movement -= SPACESHIP_SPEED_SCALAR;
    }

    if input.just_pressed(KeyCode::KeyA) || input.pressed(KeyCode::KeyA) {
        rotation += SPACESHIP_ROTATION_SCALAR * time.delta_seconds();
    } else if input.just_pressed(KeyCode::KeyD) || input.pressed(KeyCode::KeyD) {
        rotation -= SPACESHIP_ROTATION_SCALAR * time.delta_seconds();
    }

    if input.just_pressed(KeyCode::KeyQ) || input.pressed(KeyCode::KeyQ) {
        roll += SPACESHIP_ROLL_SCALAR * time.delta_seconds();
    } else if input.just_pressed(KeyCode::KeyE) || input.pressed(KeyCode::KeyE) {
        roll -= SPACESHIP_ROLL_SCALAR * time.delta_seconds();
    }

    let forward_direction = -transform.forward();
    velocity.value = forward_direction * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}