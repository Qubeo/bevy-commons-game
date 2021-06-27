use std::sync::{ Arc, Mutex };

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::mesh::Indices;
use rand::Rng;

// Q: crate:: vs. super:: ?
use crate::{ BOARD_SIZE_I, BOARD_SIZE_J };
use super::Game;
use super::Cell;

mod triangle;
mod geometry;


pub fn sample_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>
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

    let colors = [
        Color::rgb(0.286, 0.725, 0.902), // Water #49B9E6 (73, 185, 230)
        Color::rgb(0.698, 0.941, 0.329), // Grass #B2F054 (178, 240, 84)
        Color::rgb(0.722, 0.522, 0.380), // Hills ##B88561 (184, 133, 97)
    ];

    let arc_commands = Arc::new(Mutex::new(commands));
    game.board = Vec::new();

    // Generate our hex mesh
    let mesh = meshes.add(generate_hex_mesh());
    let mut rng = rand::thread_rng();
    for q in 0..BOARD_SIZE_I {

        // Push Cell row
        game.board.push(Vec::new());

        for r in 0..BOARD_SIZE_J {

            let tile = rng.gen_range(0..10);
            let tile = if tile > 0 && tile < 5 {
                0
            } else if tile >= 5 && tile < 7 {
                1
            } else {
                2
            };
            let color = colors[tile].clone();

            let height = match tile {
                0 => 0.,
                1 => 0.025 + rng.gen_range(-0.05..0.05),
                2 => 0.05 + rng.gen_range(-0.1..0.1),
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
                color,
                mesh.clone(),
                Arc::clone(&arc_commands),
                &mut materials,
            ); // .lock().unwrap().insert(Cell { height: 10.0 });

            if tile == 0 {
                arc_commands.lock().unwrap().spawn().insert(Water);
                // .with_child(Water);    // FIXME: This is probably haphazard. Oridinally: commands.with(Water)
            }
        }
    }
}

/// Spawn a hex in the world
pub fn add_hex(
    position: Vec3,
    color: Color,
    mesh: Handle<Mesh>,
    commands: Arc<Mutex<Commands>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let commands_mutex = Arc::clone(&commands);
    let mut guard = commands_mutex.lock().unwrap();
    
    guard.spawn_bundle(PbrBundle {
        mesh,
        material: materials.add(color.into()),
        transform: Transform::from_translation(position),
        ..Default::default()
    });

    // commands
}

/// Generate a single hex mesh
pub fn generate_hex_mesh() -> Mesh {
    let mut pts: Vec<[f32; 3]> = vec![];
    let c = hex::HexCoord::new(0, 0);
    geometry::bevel_hexagon_points(&mut pts, 1.0, 0.9, &c);

    let mut normals: Vec<[f32; 3]> = vec![];
    geometry::bevel_hexagon_normals(&mut normals);

    let mut uvs: Vec<[f32; 2]> = vec![];
    for _ in 0..pts.len() {
        uvs.push([0., 0.]);
    }

    let mut indices = vec![];
    geometry::bevel_hexagon_indices(&mut indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, pts);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
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