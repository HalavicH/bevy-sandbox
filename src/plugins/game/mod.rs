use crate::plugins::game::assets::GameAssetsPlugin;
use crate::plugins::game::asteroid::AsteroidPlugin;
use crate::plugins::game::blenvy::BlenvyInitializerPlugin;
use crate::plugins::game::collision::CollisionPlugin;
use crate::plugins::game::debug::DebugPlugin;
use crate::plugins::game::enemy::EnemyPlugin;
use crate::plugins::game::movement::MovementPlugin;
use crate::plugins::game::spaceship::{Spaceship, SpaceshipPlugin};
use crate::plugins::ui::hud::UiPlugin;
use bevy::prelude::*;

mod assets;
mod asteroid;
mod blenvy;
mod collision;
mod debug;
mod enemy;
mod movement;
pub mod spaceship;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.08, 0.01, 0.1)))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 1000.0,
            })
            .add_plugins(BlenvyInitializerPlugin)
            .add_plugins(GameAssetsPlugin)
            .add_plugins(SpaceshipPlugin)
            .add_plugins(AsteroidPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(CollisionPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(DebugPlugin)
            // Startup systems
            .add_systems(Startup, spawn_camera)
            // Update systems
            .add_systems(Update, fly_camera)
            .add_systems(Update, exit_on_esc_system)
            .init_resource::<DespawnTimer>()
            .register_type::<DespawnTimer>()
            .add_systems(Update, despawn_out_of_area);
    }
}

const CAMERA_DISTANCE: f32 = 200.0;

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, CAMERA_DISTANCE, 0.0))
                .looking_at(Vec3::ZERO, -Vec3::Z),
            ..Default::default()
        })
        .insert(MainCamera);
}

fn fly_camera(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Spaceship>)>,
    player_query: Query<&Transform, With<Spaceship>>,
) {
    // let mut camera_transform = camera_query.single_mut();
    // camera_transform.translation.z += -CAMERA_SPEED * time.delta_seconds();
    let player_translation: Vec3 = player_query
        .get_single()
        .map(|t| t.translation)
        .unwrap_or(Vec3::ZERO);

    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_translation + Vec3::new(0.0, CAMERA_DISTANCE, 0.0);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DespawnTimer(Timer);

impl Default for DespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DeletableByDistance {
    pub deleted: bool,
}

const DESPAWN_X: f32 = 200.0;
fn despawn_out_of_area(
    mut commands: Commands,
    mut query: Query<(Entity, &GlobalTransform, &mut DeletableByDistance)>,
    mut timer: ResMut<DespawnTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    for (e, transform, mut marked) in query.iter_mut() {
        let tr_x = &transform.translation().x;
        let distance = tr_x.abs();
        if distance <= DESPAWN_X {
            continue;
        }
        let Some(mut entity) = commands.get_entity(e) else {
            continue;
        };

        if !marked.deleted {
            marked.deleted = true;
            entity.despawn_recursive();
        }
    }
}

fn exit_on_esc_system(input: Res<ButtonInput<KeyCode>>, mut ev_wr: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        ev_wr.send(AppExit::Success);
    }
}
