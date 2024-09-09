use bevy::prelude::*;

use crate::plugins::game::spaceship::components::PlayerStats;
use crate::plugins::ui::helpers::UiBuilder;

// Health element marker
#[derive(Component, Default)]
pub struct HealthLabel;

#[derive(Component, Default)]
pub struct ScoreLabel;

#[derive(Component, Default)]
pub struct AmmoLabel;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
        app.add_systems(Update, update_player_hud_ui);
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
) {
    let list_title_style = TextStyle {
        font: asset_server.load("fonts/0xProtoNerdFont-Regular.ttf"),
        font_size: 20.0,
        ..default()
    };
    let get_list_title_style = || list_title_style.clone();

    let list_item_style = TextStyle {
        font: asset_server.load("fonts/0xProtoNerdFont-Regular.ttf"),
        font_size: 15.0,
        ..default()
    };
    let get_list_item_style = || list_item_style.clone();

    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            ..Default::default()
        })
        .with_children(|root_node| {
            root_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|stats_box| {
                    stats_box.spawn_label("Player Stats:", get_list_title_style());
                    stats_box
                        .spawn_label(
                            &format!("Health: {}", player_stats.health),
                            get_list_item_style(),
                        )
                        .insert(HealthLabel);

                    stats_box
                        .spawn_label(
                            &format!("Score: {}", player_stats.score),
                            get_list_item_style(),
                        )
                        .insert(ScoreLabel);

                    stats_box
                        .spawn_label(
                            &format!("Ammo: {}", player_stats.ammo_left),
                            get_list_item_style(),
                        )
                        .insert(AmmoLabel);
                });
        });
}

fn update_player_hud_ui(
    player_stats: Res<PlayerStats>,
    mut health_query: Query<
        &mut Text,
        (With<HealthLabel>, Without<ScoreLabel>, Without<AmmoLabel>),
    >,
    mut score_query: Query<&mut Text, (With<ScoreLabel>, Without<HealthLabel>, Without<AmmoLabel>)>,
    mut ammo_query: Query<&mut Text, (With<AmmoLabel>, Without<HealthLabel>, Without<ScoreLabel>)>,
) {
    if !player_stats.is_changed() {
        return;
    }

    for mut text in health_query.iter_mut() {
        text.sections[0].value = format!("Health: {}", player_stats.health);
    }

    for mut text in score_query.iter_mut() {
        text.sections[0].value = format!("Score: {}", player_stats.score);
    }

    for mut text in ammo_query.iter_mut() {
        text.sections[0].value = format!("Ammo: {}", player_stats.ammo_left);
    }
}
