#![feature(fn_traits)]

use bevy::{
    core::FixedTimestep,
    ecs::schedule::SystemSet,
    math,
    prelude::*,
    render::{camera::Camera, render_graph::base::camera::CAMERA_3D},
};
use bevy::diagnostic::{ FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin };
use bevy_inspector_egui::{ Inspectable, WorldInspectorPlugin };

use tokio::signal::unix::{signal, SignalKind};

// use bevy_ecs_tilemap::prelude::*;
// use bevy_easings::EasingsPlugin;
use embedded_holochain_runner::*;
use structopt::StructOpt;

use std::cmp::Ord;
use std::option::Option;
use std::cmp::PartialOrd;

const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");

use bevy_mod_picking::*;

mod assets;
mod cameras;
mod api;
mod input;
mod hextiles;
mod game;
mod ui;

use assets::{ load_assets, AssetIndex };
use api::binance::*;
use game::{board::spawn_board, player};
use game::components::{ TileType};
use hextiles::hex::{ HexCoord };
use ui::{ FontType, setup_ui };
use game::board::*;
use game::player::*;
use input::{ KeyCommandMap, GameCommandFnMap, move_player, print_keyboard_event_system, print_mouse_event_system };
use game::{ Game, BoardParams, BoardColors, GameState, Player, Bonus, Cell };
use game::bonus::{ spawn_bonus, rotate_bonus };
use cameras::{ focus_camera, setup_cameras };





// Initial configuration values
const BOARD_SIZE_I: usize = 12;
const BOARD_SIZE_J: usize = 12;

// const colors: [bevy::prelude::Color; 3] = [
const BOARD_COLORS: [bevy::prelude::Color; 3] = [
    Color::rgb(1.0, 0.858, 0.8),          // Pastel skin #FFDBCC () 
    Color::rgb(0.996, 0.882, 0.909 ),     // Pastel pink #FEE1E8 () 
    // Color::rgb(0.286, 0.725, 0.902),          // Water #49B9E6 (73, 185, 230)
    Color::rgb(0.898, 0.941, 0.629),     // Grass #B2F054 (178, 240, 84)
    // Color::rgb(0.722, 0.522, 0.380),         // Hills ##B88561 (184, 133, 97)
];

const INITIAL_BOARD_PARAMS: BoardParams = BoardParams {
    size_x: BOARD_SIZE_I,
    size_y: BOARD_SIZE_J
};

const INITIAL_BOARD_COLORS: BoardColors = BoardColors { colors: BOARD_COLORS };

#[derive(Default)]
pub struct SystemsLoaded {
    ui: bool,
    player: bool,
    tiles: bool
}

#[derive(Default)]
pub struct MeshMonkey(Handle<Mesh>);

fn main() {


    const PROFILES_DNA: &'static [u8] = include_bytes!("../../dna/workdir/profiles.dna");
    const PROJECTS_DNA: &'static [u8] = include_bytes!("../../dna/workdir/projects.dna");

    #[derive(Debug, StructOpt)]
    #[structopt(
    name = "acorn-conductor",
    about = "run the profiles dna pre-installed, and boot fine on second launch"
    )]
    struct Opt {
    #[structopt(
        default_value = "databases",
        help = "configuration values for `app_id` and `app_ws_port`
    will be overridden if an existing
    configuration is found at this path"
    )]
    datastore_path: String,

    #[structopt(long, default_value = "main-app")]
    app_id: String,

    #[structopt(long, default_value = "8888")]
    app_ws_port: u16,

    #[structopt(long, default_value = "1234")]
    admin_ws_port: u16,

    #[structopt(long, default_value = "keystore")]
    keystore_path: String,

    // community
    #[structopt(
        long,
        default_value = "kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--"
    )]
    proxy_url: String,
}

    App::build()
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<Game>()
        .init_resource::<AssetIndex>()
        .init_resource::<BinanceMarket>()
        .init_resource::<HotPrice>()
        .init_resource::<MeshMonkey>()
        .init_resource::<KeyCommandMap>()
        .init_resource::<GameCommandFnMap>()
        .init_resource::<SystemsLoaded>()
        .insert_resource(INITIAL_BOARD_PARAMS)
        .insert_resource(INITIAL_BOARD_COLORS)

        //
        .add_plugins(DefaultPlugins)
        // .add_plugin(InspectorPlugin::<HexCoord>::new())
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(DefaultPickingPlugins)
        // .add_plugin(DebugCursorPickingPlugin)
        // .add_plugin(TilemapPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        // .add_plugin(EasingsPlugin)
        //
        // .add_state(GameState::Playing)
        .add_state(GameState::Loading)
        // Beware: Need to call those two fns in the right order.        
        // .add_startup_system(input::init_key_map.system())
        .add_startup_system(input::init_command_map.system())
        .add_startup_system(cameras::setup_cameras.system())        

        // .add_startup_system(game::setup_board.system())
        // .add_startup_system(spawn_board.system())        
        // .add_startup_system(tilemap::startup_tilemap.system())
        // .add_startup_system(api::binance::setup_binance.system())
        
        
        .add_system_set(SystemSet::on_enter(GameState::Loading)
            .with_system(assets::load_assets.system())
        )
            
        
        .add_system_set(SystemSet::on_update(GameState::Loading)
            .with_system(assets::check_assets_loaded.system())
        )
        
        .add_system_set(SystemSet::on_enter(GameState::FinishedLoading)
            // .with_system(setup.system())
            // FIXME: Systems run in parallell - do a proper switch to GameState::Playing!
            .with_system(ui::setup_ui.system())
            .with_system(hextiles::sample_level.system())
            .with_system(player::spawn_player.system())
        )

        .add_system_set(SystemSet::on_update(GameState::FinishedLoading)
            .with_system(loading_finished.system())
        )

        // .add_startup_system(hextiles::sample_level.system())
        // .add_startup_system(player::spawn_player.system())

        
        // .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets.system()))
        
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(setup.system())
        )
        
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(game::player::inflate_player_by_price.system())
                .with_system(input::mouse_button_input.system())
                .with_system(input::print_keyboard_event_system.system())
                .with_system(input::print_mouse_event_system.system())
                .with_system(input::picking_events.system())
                .with_system(move_player.system())
                .with_system(focus_camera.system())
                .with_system(rotate_bonus.system())
                .with_system(ui::scoreboard_system.system())
                .with_system(ui::price_text_system.system())                
                .with_system(hextiles::water_ripple.system())
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown.system()))
        //
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(ui::display_score.system()),
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

#[derive(Bundle)]
pub struct PickaBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    #[bundle]
    pickable_bundle: PickableBundle,
    bound_volume: BoundVol
}

fn loading_finished(systems_loaded: Res<SystemsLoaded>, mut state: ResMut<State<GameState>>) {
    
    if systems_loaded.player && systems_loaded.ui && systems_loaded.tiles {
        info!("loading_finished: finished");
        state.set(GameState::Playing).unwrap();
    } else {
        warn!("loading_finished(): not yet - player: {:?}, ui: {:?}, tiles: {:?}", systems_loaded.player, systems_loaded.ui, systems_loaded.tiles);
    }
}

fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut game: ResMut<Game>,

        board_params: Res<BoardParams>,
        asset_index: Res<AssetIndex>) {
    // reset the game state
    game.cake_eaten = 0;
    game.score = 0;
    game.player.i = board_params.size_x / 2;
    game.player.j = board_params.size_y / 2;

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        light: Light {
            color: Color::rgb_u8(255, 181, 161),
            ..Default::default()
        },
        ..Default::default()
    });

    

    /* let player_scene_mesh = asset_index
        .scene_by_type
        .get(&TileType::Player)
        .unwrap()
        .clone(); */

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert_bundle(PickableBundle::default());

    let pibun: PickaBundle = PickaBundle {
        transform: Transform {
            translation: Vec3::new(
                game.player.i as f32,
                game.board[game.player.j][game.player.i].height,
                game.player.j as f32,
            ),
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
        global_transform: GlobalTransform::identity(),
        pickable_bundle: PickableBundle::default(),
        bound_volume: BoundVol::default()
    };

    /* let player_bundle = PbrBundle {
        mesh: player_mesh.clone(),
        material: materials.add(Color::rgb(0.7, 0.45, 0.77).into()),
        transform: Transform {
            translation: Vec3::new(
                game.player.i as f32,
                game.board[game.player.j][game.player.i].height,
                game.player.j as f32,
            ),
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
        ..Default::default()
    };

    // spawn the game character
    game.player.entity = Some(
        commands
            .spawn_bundle(player_bundle)
            .insert_bundle(PickableBundle::default())
            .insert(BoundVol::default())
            // .spawn_bundle(pibun)
            // .spawn_bundle(player_bundle)
            //.with_children(|cell| {
                // cell.spawn_scene(player_scene_mesh.clone()); // asset_server.load("models/AlienCake/alien.glb#Scene0"));
            //    cell.spawn_bundle(player_bundle);
            // })            
            .id(),
    );

    */

    // spawn_board(commands, game.clone(), asset_index);

    // load the scene for the cake
    game.bonus.handle = asset_index
        .scene_by_type
        .get(&TileType::Cake)
        .unwrap()
        .clone(); // asset_server.load("models/AlienCake/cakeBirthday.glb#Scene0");    
}


// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
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
