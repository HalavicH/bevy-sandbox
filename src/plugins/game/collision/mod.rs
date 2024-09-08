use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Colliders {
    pub size: Size,
    pub colliding_with: Vec<Entity>,
}

#[derive(Default, Clone, Reflect)]
#[reflect(Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Colliders {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            colliding_with: Vec::new(),
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, calculate_colliders);
    }
}

fn calculate_colliders(mut query: Query<(Entity, &GlobalTransform, &mut Colliders)>) {
    let mut map: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for (e, t, c) in query.iter() {
        for (e2, t2, c2) in query.iter() {
            if e == e2 {
                continue;
            }
            if !colliding((t, &c.size), (t2, &c2.size)) {
                continue;
            }
            map.entry(e)
                .or_insert_with(Vec::new)
                .push(e2);
        }
    }

    for (e, _, mut c) in query.iter_mut() {
        let option = map.get(&e);
        c.colliding_with = match option {
            None => Vec::new(),
            Some(vec) => vec.clone(),
        };
    }
}

fn colliding(
    (t1, s1): (&GlobalTransform, &Size),
    (t2, s2): (&GlobalTransform, &Size),
) -> bool {
    let distance = t1.translation().distance(t2.translation()) as f64;
    distance < (s1.width + s2.width) / 2f64 || distance < s1.height + s2.height / 2f64
}