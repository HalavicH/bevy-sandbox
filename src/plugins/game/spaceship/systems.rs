use bevy::prelude::*;
use crate::plugins::game::assets::GameAssets;
use crate::plugins::game::movement::components::Velocity;
use crate::plugins::game::movement::MovingObjectBundle;


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub fn spawn_spaceship(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        MovingObjectBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            acceleration: Default::default(),
            model: SceneBundle {
                scene: game_assets.get_spaceship(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                global_transform: Default::default(),
                visibility: Default::default(),
                inherited_visibility: Default::default(),
                view_visibility: Default::default(),
            }
        }
    );
}
