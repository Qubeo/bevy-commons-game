use std::sync::{ Arc, Mutex };

use bevy::scene::Entity;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::mesh::Indices;
use rand::Rng;

use bevy_mod_picking::{BoundVol, PickableBundle};

// Q: crate:: vs. super:: ?
// use crate::{ BOARD_SIZE_I, BOARD_SIZE_J };
use super::{ Game, Cell };
use crate::{BoardColors, BoardParams, SystemsLoaded};

pub mod hex;
mod geometry;


pub fn sample_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_params: Res<BoardParams>,
    board_colors: Res<BoardColors>,
    mut game: ResMut<Game>,
    mut systems_loaded: ResMut<SystemsLoaded>
) {
    // add entities to the world
    /* commands
        // camera
        .spawn_bundle( PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, 15., 0.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        });
    commands        
        // light
        .spawn_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
    */

    let arc_commands = Arc::new(Mutex::new(commands));
    game.board = Vec::new();

    // Generate our hex mesh
    let (mesh, hex_coords) = generate_hex_mesh(0.3, 1.0);
    let mesh_handle = meshes.add(mesh);
    let mut rng = rand::thread_rng();
    for q in 0..board_params.size_y {

        // Push Cell row
        game.board.push(Vec::new());

        for r in 0..board_params.size_x {

            let tile = rng.gen_range(0..10);
            let tile = if tile > 0 && tile < 5 {
                0
            } else if tile >= 5 && tile < 7 {
                1
            } else {
                2
            };
            let color = board_colors.colors[tile].clone();

            let height = match tile {
                0 => 0.05,
                1 => 0.1 + rng.gen_range(-0.05..0.05),
                2 => 0.2 + rng.gen_range(-0.1..0.1),
                _ => unreachable!(),
            };

            let pos = geometry::center(1.0, &hex::HexCoord::new(q as isize, r as isize), &[0., height, 0.]);

            // Push individual Cells
            game.board[q].push(Cell { height });

            /* game.board = (0..BOARD_SIZE_I)            // FIXME: Board size
                .map(|j| {
                    (0..BOARD_SIZE_J)
                        .map(|i| {                   
                            Cell { height: 10.0 }
                        })
                        .collect()
                })
                .collect();
            */
            
            add_hex(
                Vec3::new(pos[0], pos[1], pos[2]),
                0.2,
                color,
                mesh_handle.clone(),
                Arc::clone(&arc_commands),
                &mut materials,
            ); // .lock().unwrap().insert(Cell { height: 10.0 });

            if tile == 0 {
                arc_commands.lock().unwrap().spawn().insert(Water);
                // .with_child(Water);    // FIXME: This is probably haphazard. Oridinally: commands.with(Water)
            }
        }
    }
    systems_loaded.tiles = true;
}

/// Spawn a hex in the world
pub fn add_hex(
    position: Vec3,
    height: f32,
    color: Color,
    mesh: Handle<Mesh>,
    commands: Arc<Mutex<Commands>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> bevy::prelude::Entity {
    let commands_mutex = Arc::clone(&commands);
    let mut guard = commands_mutex.lock().unwrap();
    
    let hex_id = guard.spawn_bundle(PbrBundle {
        mesh,
        material: materials.add(color.into()),
        transform: Transform::from_translation(position),
        ..Default::default()
    })
    .insert_bundle(PickableBundle::default())
    .insert(BoundVol::default())
    .id();
    // commands

    hex_id
}

/// Generate a single hex mesh
pub fn generate_hex_mesh(height: f32, radius: f32) -> (Mesh, hex::HexCoord) {
    let mut pts: Vec<[f32; 3]> = vec![];
    let hex_coord = hex::HexCoord::new(0, 0);
    geometry::bevel_hexagon_points(&mut pts, radius, 0.925, &hex_coord, height);

    let mut normals: Vec<[f32; 3]> = vec![];
    geometry::bevel_hexagon_normals(&mut normals);

    let mut uvs: Vec<[f32; 2]> = vec![];
    for _ in 0..pts.len() {
        uvs.push([0., 0.]);
    }

    let mut indices = vec![];
    geometry::bevel_hexagon_indices(&mut indices);

    println!("gen_hex_mesh(): indices: {:?}, {:?}", indices.len(), indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, pts);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    
    (mesh, hex_coord)
}


pub struct Water;
/// Ripple water tiles slightly
pub fn water_ripple(time: Res<Time>, mut q: Query<&mut Transform, With<Water>>) {
    let time = time.seconds_since_startup() as f32;
    for mut t in q.iter_mut() {
        let (x, z) = (t.translation.x, t.translation.z);

        let ripple1 = (time / 2. + (x / 3.) + (z / 3.)).sin() * 0.1 - 0.05;
        let ripple2 = (time + (x / 3.) - (z / 4.)).cos() * 0.1 - 0.05;
        let ripple3 = (time * 2. + (x / 5.) - (z / 7.)).sin() * 0.1 - 0.05;
        t.translation = Vec3::new(x, ripple1 + ripple2 + ripple3, z);
    }
}