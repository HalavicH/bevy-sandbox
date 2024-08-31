use bevy::prelude::*;
use crate::plugins::game::collision::Colliders;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update, print_position)
            .add_systems(Update, print_colliders);
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (e, transform) in query.iter() {
        println!("Entity: {e:?} at: {:?}", transform.translation);
    }
}

fn print_colliders(query: Query<(Entity, &Colliders)>) {
    info!("New frame");
    for (e, colliders) in query.iter() {
        if colliders.colliding_with.len() > 0 {
            let vec: Vec<u32> = colliders.colliding_with.iter()
                .map(|e| e.index())
                .collect();
            println!("Entity: {:?} has collided with {:?}", e.index(), vec);
        // } else {
            // println!("Entity: {:?} has not collided with anything", e.index());
        }
    }
}
