use bevy::prelude::*;
use bevy_sandbox::plugins::game::GamePlugin;

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::srgb(0.08, 0.01, 0.1)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.1,
        })
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins(GamePlugin)
        // Startup systems
        .add_systems(Startup, spawn_camera)
        .run();
}

const CAMERA_DISTANCE: f32 = 50.0;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, CAMERA_DISTANCE, 0.0)).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}
