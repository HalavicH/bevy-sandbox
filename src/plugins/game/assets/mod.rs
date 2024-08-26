use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct GameAssets {
    spaceship: Handle<Scene>,
    asteroids: Vec<Handle<Scene>>,
    projectile: Handle<Scene>,
}

impl GameAssets {
    pub fn get_random_asteroid(&self) -> Handle<Scene> {
        let index = rand::random::<usize>() % self.asteroids.len();
        self.asteroids[index].clone()
    }

    pub fn get_spaceship(&self) -> Handle<Scene> {
        self.spaceship.clone()
    }

    pub fn get_projectile(&self) -> Handle<Scene> {
        self.projectile.clone()
    }
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
) {
    game_assets.spaceship = asset_server.load("models/Spaceship.glb#Scene0");
    game_assets.asteroids = list_asteroid_model_paths()
        .iter()
        .map(|path| asset_server.load(path))
        .collect();
    // game_assets.projectile = asset_server.load("models/projectile.gltf#Scene0");
}

fn list_asteroid_model_paths() -> Vec<String> {
    const BASE_PATH: &str = "assets/models/rock";

    let files_in_folder = std::fs::read_dir(BASE_PATH).unwrap();
    let file_paths_vec: Vec<String> = files_in_folder
        .map(|entry| entry.unwrap()
            .path()
            .to_str().unwrap()
            .to_string()
            .replace("assets/", "") + "#Scene0"
        )
        .collect();

    info!("Found {} asteroid assets\n{:#?}", file_paths_vec.len(), file_paths_vec);

    file_paths_vec
}
