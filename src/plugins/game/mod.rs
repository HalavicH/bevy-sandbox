use crate::plugins::game::assets::GameAssetsPlugin;
use crate::plugins::game::asteroid::AsteroidPlugin;
use crate::plugins::game::blenvy::BlenvyInitializerPlugin;
use crate::plugins::game::collision::CollisionPlugin;
use crate::plugins::game::movement::MovementPlugin;
use crate::plugins::game::spaceship::SpaceshipPlugin;
use bevy::prelude::*;

mod assets;
mod asteroid;
mod debug;
mod movement;
mod spaceship;
mod collision;
mod blenvy;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.08, 0.01, 0.1)))
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
            // .add_plugins(DebugPlugin)
            .add_systems(Update, exit_on_esc_system)
            .add_systems(Update, despawn_out_of_area);
    }
}

const DESPAWN_RADIUS: f32 = 200.0;
fn despawn_out_of_area(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<Camera>>,
) {
    // TODO: Relative to camera
    for (e, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_RADIUS {
            commands.entity(e)
                .despawn_recursive();
        }
    }
}

fn exit_on_esc_system(input: Res<ButtonInput<KeyCode>>, mut ev_wr: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        ev_wr.send(AppExit::Success);
    }
}
