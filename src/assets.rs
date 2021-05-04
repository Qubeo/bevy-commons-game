use bevy::prelude::*;
use std::collections::HashMap;

pub const MESH_PATHS: [&str; 3] = [
    "models/AlienCake/tile.glb#Scene0",
    "models/AlienCake/alien.glb#Scene0",
    "models/AlienCake/cakeBirthday.glb#Scene0",
];

pub const FONT_PATHS: [&str; 2] = ["fonts/FiraSans-Bold.ttf", "fonts/FiraSans-Bold.ttf"];

use super::components::{FontType, HoubaType, TileType};
pub struct HoubaTemplate {
    houba_type: HoubaType,
    // texture_path: &str,
    default_size: Vec2,
}

// Q: Differentiate materials for various types of entities, or use one big resource?
#[derive(Default)]
pub struct AssetIndex {
    pub houba_by_type: HashMap<HoubaType, Handle<ColorMaterial>>,
    pub tile_by_type: HashMap<TileType, Handle<Mesh>>,
    pub scene_by_type: HashMap<TileType, Handle<Scene>>,
    pub font_by_type: HashMap<FontType, Handle<Font>>,
}

// ARCH: Where to put this? In Houby module? In resource module? WKO question / pattern dilemma is this?
pub fn load_assets(
    mut asset_index: ResMut<AssetIndex>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scenes: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
) {
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
