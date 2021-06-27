use bevy::prelude::*;
use bevy::text::*;

use crate::SystemsLoaded;
use crate::assets::AssetIndex;

// TODO: supr:: or crate:: ?
use super::game::Game;
use super::api::binance::*;
// use crate::game::Game;

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum FontType {
    Main,
    MainBold,
}

impl Eq for FontType {}


pub fn setup_ui(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    asset_index: Res<AssetIndex>,
    mut systems_loaded: ResMut<SystemsLoaded>
) {
// scoreboard
    commands.spawn_bundle(TextBundle {
    text: Text::with_section(
        "Score:",
        TextStyle {
            font: asset_index
                .font_by_type
                .get(&FontType::Main)
                .unwrap()
                .clone(), // asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::rgb(0.5, 0.5, 1.0),
        },
        Default::default(),
    ),
    style: Style {
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..Default::default()
        },
        ..Default::default()
    },
    ..Default::default()
});

// Binance price text
    let hot_text = commands
    .spawn_bundle(TextBundle {
        text: Text::with_section(
            "BTC:USDT:",
            TextStyle {
                font: asset_index
                    .font_by_type
                    .get(&FontType::Main)
                    .unwrap()
                    .clone(), // asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::GOLD, // Color::rgb(0.5, 0.5, 1.0),
            },
                Default::default(),
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(32.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HotPrice {
            last: 0.0,
            actual: 0.0,
        });
    
    systems_loaded.ui = true;
}


// update the score displayed during the game
pub fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text, Without<HotPrice>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("Sugar Rush: {}", game.score);
}

// update the score displayed during the game
pub fn price_text_system(hot_price: Res<HotPrice>, mut query: Query<&mut Text, With<HotPrice>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("BTC:USDT: {}", hot_price.actual);
}

// display the number of cake eaten before losing
pub fn display_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    format!("Cake eaten: {}", game.cake_eaten),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
