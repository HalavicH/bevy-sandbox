use bevy::prelude::*;

// Velocity component to store movement speed
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Camera setup
    commands.spawn(Camera2dBundle::default());

    // Spawn the red square at the bottom center of the screen
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(8.0, 8.0)), // Size of the square
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, -300.0, 0.0), // Bottom center position
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Velocity { x: 0.0, y: 100.0 }); // Set a velocity moving upwards
}

fn move_square(time: Res<Time>, mut query: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let is_going_up = velocity.y > 0.0;
        let should_reverse = (is_going_up && transform.translation.y > 300.0) || (!is_going_up && transform.translation.y < -300.0);
        if should_reverse {
            velocity.y *= -1.0;
        }
        // Update the position based on velocity and time delta
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_square)
        .run();
}

