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
    let styles = Styles::init(&asset_server);

    commands
        .spawn(NodeBundle {
            style: styles.root_node.clone(),
            background_color: styles.colors.background.into(),
            ..Default::default()
        })
        .with_children(|root_node| {
            root_node
                .spawn(NodeBundle {
                    style: styles.stats_box.clone(),
                    background_color: styles.colors.stats_box.into(),
                    ..default()
                })
                .with_children(|stats_box| {
                    stats_box.spawn_label("Player Stats:", styles.list_title.clone());
                    stats_box
                        .spawn_label(
                            &format!("Health: {}", player_stats.health),
                            styles.list_item.clone(),
                        )
                        .insert(HealthLabel);

                    stats_box
                        .spawn_label(
                            &format!("Score: {}", player_stats.score),
                            styles.list_item.clone(),
                        )
                        .insert(ScoreLabel);

                    stats_box
                        .spawn_label(
                            &format!("Ammo: {}", player_stats.ammo_left),
                            styles.list_item.clone(),
                        )
                        .insert(AmmoLabel);
                });
        });
}

#[allow(clippy::type_complexity)]
fn update_player_hud_ui(
    player_stats: Res<PlayerStats>,
    mut query: Query<(
        &mut Text,
        Option<&HealthLabel>,
        Option<&ScoreLabel>,
        Option<&AmmoLabel>,
    )>,
) {
    if !player_stats.is_changed() {
        return;
    }

    for (mut text, health_label, score_label, ammo_label) in query.iter_mut() {
        if health_label.is_some() {
            text.sections[0].value = format!("Health: {}", player_stats.health);
        } else if score_label.is_some() {
            text.sections[0].value = format!("Score: {}", player_stats.score);
        } else if ammo_label.is_some() {
            text.sections[0].value = format!("Ammo: {}", player_stats.ammo_left);
        }
    }
}

pub struct Colors {
    pub background: Color,
    pub stats_box: Color,
}

pub struct Styles {
    pub colors: Colors,
    pub root_node: Style,
    pub stats_box: Style,
    pub list_title: TextStyle,
    pub list_item: TextStyle,
}

impl Styles {
    pub fn init(asset_server: &Res<AssetServer>) -> Self {
        Self {
            colors: Colors {
                background: Color::srgba(0.0, 0.0, 0.0, 0.1),
                stats_box: Color::srgba(0.0, 0.0, 0.0, 0.5),
            },
            root_node: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            stats_box: Style {
                width: Val::Px(200.),
                border: UiRect::all(Val::Px(2.)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            list_title: TextStyle {
                font: asset_server.load("fonts/0xProtoNerdFont-Regular.ttf"),
                font_size: 20.0,
                ..default()
            },
            list_item: TextStyle {
                font: asset_server.load("fonts/0xProtoNerdFont-Regular.ttf"),
                font_size: 15.0,
                ..default()
            },
        }
    }
}