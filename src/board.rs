use super::{BOARD_SIZE_I, BOARD_SIZE_J};
use bevy::prelude::*;
use rand::Rng;

use super::{assets::AssetIndex, components::TileType, Cell, Game};

pub fn spawn_board(mut commands: Commands, mut game: ResMut<Game>, asset_index: Res<AssetIndex>) {
    // spawn the game board
    let cell_scene = asset_index.scene_by_type.get(&TileType::Square).unwrap(); // asset_server.load("models/AlienCake/tile.glb#Scene0");

    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let height = rand::thread_rng().gen_range(-0.1..0.1);
                    commands
                        .spawn_bundle((
                            Transform::from_xyz(i as f32, height - 0.2, j as f32),
                            GlobalTransform::identity(),
                        ))
                        .with_children(|cell| {
                            cell.spawn_scene(cell_scene.clone());
                        });
                    Cell { height }
                })
                .collect()
        })
        .collect();
}
