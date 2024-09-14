use crate::plugins::game::assets::GameAssetsPlugin;
use crate::plugins::game::asteroid::AsteroidPlugin;
use crate::plugins::game::blenvy::BlenvyInitializerPlugin;
use crate::plugins::game::collision::CollisionPlugin;
use crate::plugins::game::enemy::EnemyPlugin;
use crate::plugins::game::movement::MovementPlugin;
use crate::plugins::game::spaceship::components::Spaceship;
use crate::plugins::game::spaceship::SpaceshipPlugin;
use crate::plugins::ui::hud::UiPlugin;
use bevy::prelude::*;
use crate::plugins::game::debug::DebugPlugin;

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
        app
            // .insert_resource(ClearColor(Color::srgb(0.08, 0.01, 0.1)))
            // .insert_resource(AmbientLight {
            //     color: Color::default(),
            //     brightness: 1000.0,
            // })
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
            // .add_systems(Startup, spawn_camera)
            // Update systems
            // .add_systems(Update, fly_camera)
            .add_systems(Update, exit_on_esc_system)
            .add_systems(Update, despawn_out_of_area);
    }
}

const CAMERA_DISTANCE: f32 = 200.0;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct MainBlenderCamera;

// fn spawn_camera(mut commands: Commands) {
//     commands
//         .spawn(Camera3dBundle {
//             transform: Transform::from_translation(Vec3::new(0.0, CAMERA_DISTANCE, 0.0))
//                 .looking_at(Vec3::ZERO, -Vec3::Z),
//             ..Default::default()
//         })
//         .insert(MainCamera);
// }
//
// fn fly_camera(
//     mut camera_query: Query<&mut Transform, (With<MainBlenderCamera>, Without<Spaceship>)>,
//     player_query: Query<&Transform, With<Spaceship>>,
// ) {
//     // let mut camera_transform = camera_query.single_mut();
//     // camera_transform.translation.z += -CAMERA_SPEED * time.delta_seconds();
//     let player_translation: Vec3 = player_query
//         .get_single()
//         .map(|t| t.translation)
//         .unwrap_or(Vec3::ZERO);
//
//     let mut camera_transform = camera_query.iter_mut();
//     for mut camera in camera_transform {
//         camera.translation.z = player_translation.z;
//     }
// }

const DESPAWN_X: f32 = 200.0;
fn despawn_out_of_area(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Camera>>,
) {
    // TODO: Relative to camera
    for (e, transform) in query.iter() {
        let tr_x = &transform.translation().x;
        let distance = tr_x.abs();
        if distance > DESPAWN_X {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn exit_on_esc_system(input: Res<ButtonInput<KeyCode>>, mut ev_wr: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        ev_wr.send(AppExit::Success);
    }
}
