use bevy::prelude::*;

use super::{Game, BOARD_SIZE_I, BOARD_SIZE_J};

// control the game character
pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut moved = false;
    let mut rotation = 0.0;
    if keyboard_input.just_pressed(KeyCode::Up) {
        if game.player.i < BOARD_SIZE_I - 1 {
            game.player.i += 1;
        }
        rotation = -std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        if game.player.i > 0 {
            game.player.i -= 1;
        }
        rotation = std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        if game.player.j < BOARD_SIZE_J - 1 {
            game.player.j += 1;
        }
        rotation = std::f32::consts::PI;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        if game.player.j > 0 {
            game.player.j -= 1;
        }
        rotation = 0.0;
        moved = true;
    }

    // move on the board
    if moved {
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(
                game.player.i as f32,
                game.board[game.player.j][game.player.i].height,
                game.player.j as f32,
            ),
            rotation: Quat::from_rotation_y(rotation),
            ..Default::default()
        };
    }

    // eat the cake!
    if let Some(entity) = game.bonus.entity {
        if game.player.i == game.bonus.i && game.player.j == game.bonus.j {
            game.score += 2;
            game.cake_eaten += 1;
            commands.entity(entity).despawn_recursive();
            game.bonus.entity = None;
        }
    }
}

pub fn spawn_mesh() {
    
}
