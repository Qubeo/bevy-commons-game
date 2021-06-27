use bevy::prelude::*;
use std::collections::HashMap;
use bevy::asset::LoadState;

pub const MESH_PATHS: [&str; 5] = [
    "models/AlienCake/tile.glb#Scene0",
    "models/AlienCake/alien.glb#Scene0",
    "models/AlienCake/cakeBirthday.glb#Scene0",
    "models/characters/monkey.gltf#Mesh0/Primitive0",
    // "models/characters/fox/scene.gltf#Mesh0/Primitive0"
    "models/characters/bunnylamp/bunnylamp.gltf#Mesh0/Primitive0"
    // "models/AlienCake/alien.glb#Scene0/Mesh0"
];

pub const FONT_PATHS: [&str; 2] = ["fonts/FiraSans-Bold.ttf", "fonts/FiraSans-Bold.ttf"];

use super::game::components::{ TileType };
use super::game::player::{ CharacterType };
use super::ui::{ FontType };
use crate::MeshMonkey;
use crate::game::GameState;

/* pub struct HoubaTemplate {
    houba_type: HoubaType,
    // texture_path: &str,
    default_size: Vec2,
} */

// Q: Differentiate materials for various types of entities, or use one big resource?
#[derive(Default)]
pub struct AssetIndex {
    // pub houba_by_type: HashMap<HoubaType, Handle<ColorMaterial>>,
    pub tile_by_type: HashMap<TileType, Handle<Mesh>>,
    pub scene_by_type: HashMap<TileType, Handle<Scene>>,
    pub font_by_type: HashMap<FontType, Handle<Font>>,
    pub mesh_by_type: HashMap<CharacterType, Handle<Mesh>>
}

// ARCH: Where to put this? In Houby module? In resource module? WKO question / pattern dilemma is this?
// TODO: Implement loading state conditioned stage transitions via "asset_server.get_load_state(handle) == LoadState::Loaded"?
pub fn load_assets(
    mut asset_index: ResMut<AssetIndex>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scenes: ResMut<Assets<Scene>>,
    mut mesh_monkey: ResMut<MeshMonkey>,
    asset_server: Res<AssetServer>,
) {

    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("models").unwrap();
    let mesh_handle: Handle<Mesh> = asset_server.get_handle("models/characters/bunnylamp/bunnylamp.gltf#Mesh0/Primitive0");
    // let mesh_handle: Handle<Mesh> = asset_server.get_handle("models/characters/fox/scene.gltf#Mesh0/Primitive0"); // "models/Monkey.gltf#Mesh0/Primitive0");
    // let mesh_handle: Handle<Mesh> = asset_server.get_handle("models/characters/monkey.gltf#Mesh0/Primitive0");

    mesh_monkey.0 = mesh_handle.clone();

    asset_index
        .mesh_by_type
        .insert(CharacterType::Monkey, asset_server.load(MESH_PATHS[3]).into());

    asset_index
        .mesh_by_type
        .insert(CharacterType::Bunny, asset_server.load(MESH_PATHS[4]).into());

    asset_index
        .scene_by_type
        .insert(TileType::Square, asset_server.load(MESH_PATHS[0]).into());

    asset_index
        .scene_by_type
        .insert(TileType::Player, asset_server.load(MESH_PATHS[1]).into());
    

    asset_index
        .scene_by_type
        .insert(TileType::Cake, asset_server.load(MESH_PATHS[2]).into());
        

    // Load fonts
    asset_index
        .font_by_type
        .insert(FontType::Main, asset_server.load(FONT_PATHS[1]).into());
}


pub fn check_assets_loaded (
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    asset_index: Res<AssetIndex>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(asset_index.mesh_by_type.iter().map(|(_, handle)| { handle.id })) {
        println!("Meshes loaded!");
        state.set(GameState::FinishedLoading).unwrap();
    }

    /* if let LoadState::Loaded =
        asset_server.get_group_load_state(mesh_handles.iter().map(|(handle, _)| handle))
    {
        println!("check_assets_loaded: finished loading");
        // mesh_handles.iter().map(|h| { println!("handle: {:?}", h); });
        state.set(GameState::FinishedLoading).unwrap();
    } else {
        println!("checking assets loaded: not loaded yet");
    } */
}
