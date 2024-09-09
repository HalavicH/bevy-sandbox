use crate::plugins::game::assets::GameAssets;
use crate::plugins::game::collision::{Colliders, Size};
use crate::plugins::game::movement::components::Velocity;
use crate::plugins::game::movement::MovingObjectBundle;
use crate::plugins::game::spaceship::{PlayerStats, ProjectileTimer, Spaceship};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;
use crate::plugins::game::DeletableByDistance;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Projectile {
    pub weapon_type: WeaponType,
    pub spawned_at: Vec3,
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, Reflect)]
pub enum WeaponType {
    #[default]
    Bullet,
    Missile,
}

impl WeaponType {
    pub fn new_weapon_with_default_stats(&self) -> Weapon {
        match self {
            WeaponType::Bullet => Weapon {
                weapon_type: WeaponType::Bullet,
                ammo_left: 100,
                max_ammo: 100,
            },
            WeaponType::Missile => Weapon {
                weapon_type: WeaponType::Missile,
                ammo_left: 50,
                max_ammo: 50,
            },
        }
    }

    pub fn get_rounds_per_minute(&self) -> f32 {
        match self {
            WeaponType::Bullet => 300.0,
            WeaponType::Missile => 60.0,
        }
    }

    pub fn get_delay_between_shots_sec(&self) -> Duration {
        Duration::from_secs_f32(1. / self.get_rounds_per_minute() * 60.0)
    }

    pub fn get_damage(&self) -> i32 {
        match self {
            WeaponType::Bullet => 10,
            WeaponType::Missile => 50,
        }
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            WeaponType::Bullet => 100.0,
            WeaponType::Missile => 50.0,
        }
    }

    pub fn get_range(&self) -> f32 {
        match self {
            WeaponType::Bullet => 100.0,
            WeaponType::Missile => 50.0,
        }
    }

    pub fn get_scale(&self) -> f32 {
        match self {
            WeaponType::Bullet => 0.1,
            WeaponType::Missile => 1.,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            WeaponType::Bullet => "Bullet".to_string(),
            WeaponType::Missile => "Missile".to_string(),
        }
    }
}

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub ammo_left: i32,
    pub max_ammo: i32,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .add_systems(Update, fire_weapon);
    }
}
pub fn fire_weapon(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    query: Query<&Transform, With<Spaceship>>,
    input: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<ProjectileTimer>,
    time: Res<Time>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let Ok(spaceship_transform) = query.get_single() else {
        debug!("Spaceship not found or multiple spaceships found");
        return;
    };

    let weapon_type = &player_stats.active_weapon.clone();
    let Some(weapon) = player_stats.weapons.get_mut(weapon_type) else {
        warn!("Weapon {:?} not initialized", weapon_type);
        return;
    };

    timer.value.tick(time.delta());
    if !input.pressed(KeyCode::Space) {
        return;
    }

    if timer.value.elapsed() < weapon_type.get_delay_between_shots_sec() {
        return;
    }
    timer.value.reset();

    weapon.ammo_left -= 1;

    // We negate the forward direction because bevy inverts the z-axis
    let spaceship_forward_direction = spaceship_transform.forward();

    let transform = calc_projectile_transform(spaceship_transform, spaceship_forward_direction)
        .with_scale(Vec3::splat(weapon_type.get_scale()));

    let handle = game_assets.get_projectile();
    // let model_size = game_assets.get_model_size(handle);
    let colliders = Colliders::new(Size {
        width: 1.0,
        height: 3.0,
    });
    let projectile_velocity = spaceship_forward_direction * weapon_type.get_speed();
    let spawned_at = spaceship_transform.translation;
    commands
        .spawn((
            MovingObjectBundle {
                velocity: Velocity::new(projectile_velocity),
                acceleration: Default::default(),
                model: SceneBundle {
                    scene: handle,
                    transform,
                    ..SceneBundle::default()
                },
                colliders,
            },
            DeletableByDistance { deleted: false }, // Very bad workaround. TODO: Fix
            Projectile {
                weapon_type: weapon_type.clone(),
                spawned_at,
            },
            Name::new(weapon_type.to_string()),
        ));
}

const NOSE_OFFSET: f32 = 8.0;

fn calc_projectile_transform(
    spaceship_transform: &Transform,
    spaceship_forward_direction: Dir3,
) -> Transform {
    let spaceship_nose_vec = spaceship_forward_direction * NOSE_OFFSET;
    let mut transform =
        Transform::from_translation(spaceship_transform.translation + spaceship_nose_vec);

    // Rotate the projectile 90 degrees around the X-axis
    transform.rotation = spaceship_transform.rotation * Quat::from_rotation_x(-FRAC_PI_2);
    transform
}
