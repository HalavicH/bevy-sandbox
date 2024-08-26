use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

pub fn print_position(query: Query<(Entity, &Transform)>) {
    for (e, transform) in query.iter() {
        println!("Entity: {e:?} at: {:?}", transform.translation);
    }
}
