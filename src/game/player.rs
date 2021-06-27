
use bevy::prelude::*;
use bevy_mod_picking::{ PickableBundle, BoundVol };

use crate::SystemsLoaded;
use crate::game::{ Game };
use crate::api::binance::{ HotPrice };
use crate::assets::{ AssetIndex };

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
// TODO: Tease out the (logical) levels - graphical representation vs. the conceptual / game logic level.
pub enum CharacterType {
    Monkey,
    Bunny,
    Alien,
    Fox
}
impl Eq for CharacterType {}


pub fn spawn_player(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_index: Res<AssetIndex>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut systems_loaded: ResMut<SystemsLoaded>
    ) {
    
    if let Some(player_mesh) = asset_index
    .mesh_by_type
    .get(&CharacterType::Bunny) {

        let mut character_transform = Transform::from_xyz(9.0, 0.8, 8.0); 
        // character_transform.apply_non_uniform_scale(Vec3::new(0.1, 0.1, 0.1));
        character_transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
        // apply_non_uniform_scale(Vec3::new(0.1, 0.1, 0.1));
    
        game.player.entity = Some(    
            commands.spawn_bundle(PbrBundle {
                mesh: player_mesh.clone(), // meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
                transform: character_transform,
                global_transform: GlobalTransform::identity(),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(BoundVol::default())
            .id()
        );        
    } else {
        println!("spawn_player: Player mesh asset not loaded yet.");
        // TODO: Do something to retry.
    }
    
    systems_loaded.player = true;
}

pub(crate) fn inflate_player_by_price(
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
            player_transform.scale = Vec3::splat(0.2);
            /* Vec3::splat(
                price_modulated_actual, // * time.seconds_since_startup().sin() as f32).abs(),
            ); */
            
            /* player_transform.scale.lerp(
                Vec3::splat(price_modulated_actual + 0.1),
                1.0 // time.seconds_since_startup() as f32,
            ); */

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