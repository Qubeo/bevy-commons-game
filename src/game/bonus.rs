use bevy::prelude::*;

use crate::game::{ Game, GameState };
use crate::{ BOARD_SIZE_J, BOARD_SIZE_I };
use crate::PickaBundle;

use bevy_mod_picking::{BoundVol, PickableBundle};

use rand::Rng;

// despawn the bonus if there is one, then spawn a new one at a random location
// Run criteria: fixed time step
pub fn spawn_bonus(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut asset_server: ResMut<AssetServer>
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
    let pibun: PickaBundle = PickaBundle {
        transform: Transform {
            translation: Vec3::new(
                game.bonus.i as f32,
                game.board[game.player.j][game.player.i].height + 0.2,
                game.bonus.j as f32,
            ),
            ..Default::default()
        },
        global_transform: GlobalTransform::identity(),
        pickable_bundle: PickableBundle::default(),
        bound_volume: BoundVol::default()
    };

    
    game.bonus.entity = Some(
        commands
            .spawn_bundle(pibun)
            .with_children(|cell| {
                cell.spawn_scene(game.bonus.handle.clone());
                // cell.spawn_bundle(bobun)
            }).insert_bundle(PickableBundle::default())
            .id(),
    );
}

// let the cake turn on itself
pub fn rotate_bonus(game: Res<Game>, time: Res<Time>, mut transforms: Query<&mut Transform>) {
    if let Some(entity) = game.bonus.entity {
        if let Ok(mut cake_transform) = transforms.get_mut(entity) {
            cake_transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
            let scale = 1.0 + (/* game.score as f32 / 1.0 * */ time.seconds_since_startup().sin() as f32).abs();
            // info!("rotate_bonus:: scale:: {:?}", scale);
            cake_transform.scale = Vec3::splat(
                scale
            );
        }
    }
}
