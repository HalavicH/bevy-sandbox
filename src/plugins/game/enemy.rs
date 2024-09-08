
use bevy::prelude::*;
use log::info;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct EnemySpaceship;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct EyeBotEnemy {
    pub path_radius: f32,
    pub initial_position: Option<Vec3>,
    pub orbit_vector: Vec3,
}

// #[derive(Component, Default, Clone, Reflect)]
// #[reflect(Component, Default)]
// pub struct EnemySpaceship;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource()
            .add_systems(Update, (move_eye_bot_enemy));
    }
}

const EYE_BOT_ENEMY_PATH_RADIUS: f32 = 10.0;

#[derive(Resource, Debug)]
pub struct EyeBotCircleTimer {
    pub timer: Timer,
}

fn rotate_eye_bot_enemy_orbit_vector(orbit_vector: &mut Vec3, time: &Time) {
    let rotation = Quat::from_rotation_y(time.delta_seconds());
    *orbit_vector = rotation.mul_vec3(*orbit_vector);
}


pub fn move_eye_bot_enemy(
    mut query: Query<(&mut Transform, &GlobalTransform, &mut EyeBotEnemy)>,
    time: Res<Time>
) {
    for (mut transform, gt, mut eb) in query.iter_mut() {
        if eb.initial_position.is_none() {
            eb.initial_position = Some(gt.translation());
        }

        rotate_eye_bot_enemy_orbit_vector(&mut eb.orbit_vector, &time);
        transform.translation = eb.initial_position.unwrap() + eb.orbit_vector * EYE_BOT_ENEMY_PATH_RADIUS;
        info!("Moving EyeBotEnemy. Translation: {:?}, global translation: {:?}", transform.translation, gt.translation());
    }
}