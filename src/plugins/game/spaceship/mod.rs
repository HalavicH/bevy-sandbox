use bevy::prelude::*;
use std::collections::HashMap;

pub mod weapon;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resource initialization
            .init_resource::<ProjectileTimer>()
            .init_resource::<PlayerStats>()
            // Type registration
            .register_type::<Spaceship>()
            .register_type::<PlayerStats>()
            // Systems
            .add_systems(Update, move_spaceship)
            // Plugins
            .add_plugins(WeaponPlugin);
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Spaceship;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Debug)]
pub struct PlayerStats {
    pub score: i32,
    pub health: i32,
    pub active_weapon: WeaponType,
    pub weapons: HashMap<WeaponType, Weapon>,
}

impl Default for PlayerStats {
    fn default() -> Self {
        let mut weapons = HashMap::default();
        weapons.insert(
            WeaponType::Missile,
            WeaponType::Missile.new_weapon_with_default_stats(),
        );
        Self {
            score: 0,
            health: 100,
            active_weapon: WeaponType::Missile,
            weapons,
        }
    }
}

use crate::plugins::game::collision::{Colliders, Size};
use crate::plugins::game::movement::components::{Acceleration, Velocity};
use crate::plugins::game::spaceship::weapon::{Weapon, WeaponPlugin, WeaponType};
use blenvy::{BlueprintInfo, SpawnBlueprint};

const SPACESHIP_SPEED_SCALAR: f32 = 25.0;
const SPACESHIP_ROTATION_SCALAR: f32 = 2.5;
const SPACESHIP_ROLL_SCALAR: f32 = 5.0;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
struct LazyLoad;

pub fn _spawn_spaceship_from_blueprint_example(mut commands: Commands) {
    const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
    const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    commands
        .spawn((
            BlueprintInfo {
                name: "Spaceship".into(),
                path: "blueprints/Spaceship.glb".into(),
            },
            SpawnBlueprint,
            Velocity {
                value: STARTING_VELOCITY,
            },
            Acceleration::default(),
            Colliders::new(Size {
                width: 5.0,
                height: 5.0,
            }),
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
    let forward_direction = transform.forward();

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
        let max_cooldown = 100.;
        Self {
            value: Timer::from_seconds(max_cooldown, TimerMode::Repeating),
        }
    }
}
