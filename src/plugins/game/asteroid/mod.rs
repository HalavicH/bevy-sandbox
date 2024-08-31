use std::ops::Range;
use bevy::prelude::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::plugins::game::assets::GameAssets;
use crate::plugins::game::movement::components::{Acceleration, Velocity};
use crate::plugins::game::movement::MovingObjectBundle;

pub struct AsteroidPlugin;

const SPAWN_RANGE_X: Range<f32> = -100.0..100.0;
const SPAWN_RANGE_Y: Range<f32> = -100.0..100.0;

const SPAWN_RANGE_EXCLUSION_X: Range<f32> = -50.0..50.0;
const SPAWN_RANGE_EXCLUSION_Y: Range<f32> = -50.0..50.0;

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
    game_assets: Res<GameAssets>,
) {
    timer.timer.tick(time.delta());
    if !(timer.timer.finished()) {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = gen_asteroid_position(&mut rng);

    let mut random_unit_vector = || {
        let quarter = match (translation.x, translation.z) {
            (x, z) if x > 0. && z > 0. => 0,
            (x, z) if x < 0. && z > 0. => 1,
            (x, z) if x < 0. && z < 0. => 2,
            (x, z) if x > 0. && z < 0. => 3,
            _ => -1,
        };

        if quarter == -1 {
            return Vec3::ZERO;
        }

        Vec3::new(
            rng.gen_range(-1.0..1.0),
            0.,
            rng.gen_range(-1.0..1.0),
        ).normalize_or_zero()
    };

    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn(MovingObjectBundle {
        velocity: Velocity { value: velocity },
        acceleration: Acceleration { value: acceleration },
        model: SceneBundle {
            scene: game_assets.get_random_asteroid(),
            transform: Transform::from_translation(translation),
            ..SceneBundle::default()
        },
    })
        .insert(Asteroid);
}

fn gen_asteroid_position(mut rng: &mut ThreadRng) -> Vec3 {
    let mut pos = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Y),
    );

    while SPAWN_RANGE_EXCLUSION_X.contains(&pos.x) && SPAWN_RANGE_EXCLUSION_Y.contains(&pos.z) {
        pos.x = rand_within_range(SPAWN_RANGE_X);
        pos.z = rand_within_range(SPAWN_RANGE_Y);
    }
    pos
}

fn rand_within_range(range: Range<f32>) -> f32 {
    let delta = range.end - range.start;
    range.start + delta * rand::random::<f32>()
}