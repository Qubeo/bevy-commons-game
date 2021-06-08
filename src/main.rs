use bevy::{
    core::FixedTimestep,
    ecs::schedule::SystemSet,
    math,
    prelude::*,
    render::{camera::Camera, render_graph::base::camera::CAMERA_3D},
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy_ecs_tilemap::prelude::*;
// use bevy_easings::EasingsPlugin;
use rand::Rng;
// use embedded_holochain_runner::*;
// use structopt::StructOpt;
// const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");

mod assets;
mod api;
mod board;
mod components;
mod input;
mod tilemap;
mod hextiles;
mod game;

use assets::{load_assets, AssetIndex};
use api::binance::*;
use board::spawn_board;
use components::{FontType, HoubaType, TileType};
use input::move_player;
use tilemap::startup_tilemap;
use game::{ Game, GameState, Player, Bonus, Cell };

fn main() {

    App::build()
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<Game>()
        .init_resource::<AssetIndex>()
        .init_resource::<BinanceMarket>()
        .init_resource::<HotPrice>()
        //
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        // .add_plugin(EasingsPlugin)
        //
        .add_state(GameState::Playing)
        //
        .add_startup_system(setup_cameras.system())
        .add_startup_system(load_assets.system())
        // .add_startup_system(spawn_board.system())        
        // .add_startup_system(tilemap::startup_tilemap.system())
        .add_startup_system(api::binance::setup_binance.system())
        .add_startup_system(hextiles::sample_level.system())
        //
        // .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets.system()))
        //
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(inflate_player_by_price.system())
                .with_system(move_player.system())
                .with_system(focus_camera.system())
                .with_system(rotate_bonus.system())
                .with_system(scoreboard_system.system())
                .with_system(price_text_system.system())                
                .with_system(hextiles::water_ripple.system())
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown.system()))
        //
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(display_score.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(gameover_keyboard.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown.system()))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(5.0))
                .with_system(spawn_bonus.system())
                .with_system(refresh_binance_data.system())
        )
        .run();
}


const BOARD_SIZE_I: usize = 15;
const BOARD_SIZE_J: usize = 15;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I as f32 / 2.0,
    0.0,
    BOARD_SIZE_J as f32 / 2.0 - 0.5,
];

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = Vec3::from(RESET_FOCUS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.0),
            2.0 * BOARD_SIZE_J as f32 / 3.0,
            BOARD_SIZE_J as f32 / 2.0 - 0.5,
        )
        .looking_at(game.camera_is_focus, Vec3::Y),
        ..Default::default()
    });
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup(mut commands: Commands, mut game: ResMut<Game>, asset_index: Res<AssetIndex>) {
    // reset the game state
    game.cake_eaten = 0;
    game.score = 0;
    game.player.i = BOARD_SIZE_I / 2;
    game.player.j = BOARD_SIZE_J / 2;

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..Default::default()
    });

    let player_scene_mesh = asset_index
        .scene_by_type
        .get(&TileType::Player)
        .unwrap()
        .clone();

    // spawn the game character
    game.player.entity = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(
                        game.player.i as f32,
                        game.board[game.player.j][game.player.i].height,
                        game.player.j as f32,
                    ),
                    rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell| {
                cell.spawn_scene(player_scene_mesh.clone()); // asset_server.load("models/AlienCake/alien.glb#Scene0"));
            })
            .id(),
    );

    // spawn_board(commands, game.clone(), asset_index);

    // load the scene for the cake
    game.bonus.handle = asset_index
        .scene_by_type
        .get(&TileType::Cake)
        .unwrap()
        .clone(); // asset_server.load("models/AlienCake/cakeBirthday.glb#Scene0");

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
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// change the focus of the camera
fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
) {
    const SPEED: f32 = 0.1;
    // if there is both a player and a bonus, target the mid-point of them
    if let (Some(player_entity), Some(bonus_entity)) = (game.player.entity, game.bonus.entity) {
        if let (Ok(player_transform), Ok(bonus_transform)) = (
            transforms.q1().get(player_entity),
            transforms.q1().get(bonus_entity),
        ) {
            game.camera_should_focus = player_transform
                .translation
                .lerp(bonus_transform.translation, 0.1);
        }
    // otherwise, if there is only a player, target the player
    } else if let Some(player_entity) = game.player.entity {
        if let Ok(player_transform) = transforms.q1().get(player_entity) {
            game.camera_should_focus = player_transform.translation;
        }
    // otherwise, target the middle
    } else {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }
    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        game.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for (mut transform, camera) in transforms.q0_mut().iter_mut() {
        if camera.name == Some(CAMERA_3D.to_string()) {
            *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
        }
    }
}

// despawn the bonus if there is one, then spawn a new one at a random location
fn spawn_bonus(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    if *state.current() != GameState::Playing {
        return;
    }
    if let Some(entity) = game.bonus.entity {
        game.score -= 3;
        commands.entity(entity).despawn_recursive();
        game.bonus.entity = None;
        if game.score <= -180 {
            state.set(GameState::GameOver).unwrap();
            return;
        }
    }

    // ensure bonus doesn't spawn on the player
    loop {
        game.bonus.i = rand::thread_rng().gen_range(0..BOARD_SIZE_I);
        game.bonus.j = rand::thread_rng().gen_range(0..BOARD_SIZE_J);
        if game.bonus.i != game.player.i || game.bonus.j != game.player.j {
            break;
        }
    }
    game.bonus.entity = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(
                        game.bonus.i as f32,
                        game.board[game.player.j][game.player.i].height + 0.2,
                        game.bonus.j as f32,
                    ),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell| {
                cell.spawn_scene(game.bonus.handle.clone());
            })
            .id(),
    );
}

// let the cake turn on itself
fn rotate_bonus(game: Res<Game>, time: Res<Time>, mut transforms: Query<&mut Transform>) {
    if let Some(entity) = game.bonus.entity {
        if let Ok(mut cake_transform) = transforms.get_mut(entity) {
            cake_transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
            cake_transform.scale = Vec3::splat(
                1.0 + (game.score as f32 / 1.0 * time.seconds_since_startup().sin() as f32).abs(),
            );
        }
    }
}

fn inflate_player_by_price(
    price: Res<HotPrice>,
    game: Res<Game>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    if let Some(entity) = game.player.entity {
        if let Ok(mut player_transform) = transforms.get_mut(entity) {
            // player_transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
            let price_modulated_last: f32 =
                (((price.actual * 10000.0) % 1.0).powf(3.0) * 500.0) as f32;
            let price_modulated_actual: f32 =
                (((price.actual * 10000.0) % 1.0).powf(3.0) * 10.0) as f32;

            // entity.
            player_transform.scale =
            /* Vec3::splat(
                price_modulated_actual, // * time.seconds_since_startup().sin() as f32).abs(),
            ); */
            
            player_transform.scale.lerp(
                Vec3::splat(price_modulated_actual),
                1.0 // time.seconds_since_startup() as f32,
            );

            /* player_transform.scale = Vec3::splat(
                    price_modulated_actual, // * time.seconds_since_startup().sin() as f32).abs(),
                );
            } */
            /*  .ease_to(
                    Sprite {
                        size: Vec2::new(100., 100.),
                        ..Default::default()
                    },
                    EaseFunction::QuadraticIn,
                    EasingType::PingPong {
                        duration: std::time::Duration::from_secs(1),
                        pause: std::time::Duration::from_millis(500),
                    },
            */
        }
    }
}

// update the score displayed during the game
fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text, Without<HotPrice>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("Sugar Rush: {}", game.score);
}

// update the score displayed during the game
fn price_text_system(hot_price: Res<HotPrice>, mut query: Query<&mut Text, With<HotPrice>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("BTC:USDT: {}", hot_price.actual);
}

// restart the game when pressing spacebar
fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

fn refresh_binance_data(
    hot_price: ResMut<api::binance::HotPrice>,
    binance_api: Res<BinanceMarket>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    api::binance::get_price(hot_price, binance_api);
    if keyboard_input.just_pressed(KeyCode::K) {
        println!("refresh_binance_data: K pressed: ");
    }
}

// display the number of cake eaten before losing
fn display_score(
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
