use bevy::prelude::*;
use bevy_sandbox::plugins::game::GamePlugin;

fn main() {
    App::new()
        // Bevy built-ins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins(GamePlugin)
        .run();
}
