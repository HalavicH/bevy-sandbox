use std::ops::Range;
use bevy::prelude::*;
use rand::Rng;
use crate::components::{Acceleration, Velocity};
use crate::plugins::game::movement::MovingObjectBundle;

pub struct AsteroidPlugin;

const SPAWN_RANGE_X: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_Y: Range<f32> = -50.0..50.0;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 5.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AsteroidSpawnTimer>()
            .add_systems(Update, spawn_asteroid);
    }
}

pub fn spawn_asteroid(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    asset_server: Res<AssetServer>,

) {
    timer.timer.tick(time.delta());
    if !(timer.timer.finished()) {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Y)
    );

    let mut random_unit_vector = || Vec3::new(
        rng.gen_range(-1.0..1.0),
        0.,
        rng.gen_range(-1.0..1.0)
    ).normalize_or_zero();

    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn(MovingObjectBundle {
        velocity: Velocity { value: velocity },
        acceleration: Acceleration { value: acceleration },
        model: SceneBundle {
            scene: asset_server.load(random_asteroid_model_path_str()),
            transform: Transform::from_translation(translation),
            ..SceneBundle::default()
        }
    })
    .insert(Asteroid);
}

fn random_asteroid_model_path_str() -> String {
    const BASE_PATH: &str = "assets/models/rock";
    // pick any dlb file in the folder
    let files_in_folder = std::fs::read_dir(BASE_PATH).unwrap();
    let file_paths_vec: Vec<String> = files_in_folder
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect();

    let random_index = rand::random::<usize>() % file_paths_vec.len();
    let random_file_path = &file_paths_vec[random_index];
    println!("Random file path: {:?}", random_file_path);
    random_file_path.replace("assets/", "") + "#Scene0"
}

fn rand_within_range(range: Range<f32>) -> f32 {
    let delta = range.end - range.start;
    range.start + delta * rand::random::<f32>()
}