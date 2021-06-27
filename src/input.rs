
use std::sync::{Arc, Mutex};

use bevy::{prelude::*, utils::HashMap, input::keyboard::KeyboardInput, input::mouse::MouseButtonInput };
use bevy_mod_picking::{PickingEvent, SelectionEvent};
use rand::Rng;

use crate::hextiles::{add_hex, generate_hex_mesh};
use super::{ Game, BOARD_SIZE_I, BOARD_SIZE_J };

// use lazy_static::lazy_static;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum GameCommand {
    CREATE_HEX, // (Vec3),
    PLAYER_MOVE,
    SELECTED_ENTITY_MOVE
}
impl Eq for GameCommand {}

type CmdFn = Fn(u32) -> ();

#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyCommandMap(HashMap<MouseButton, GameCommand>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameCommandFnMap {
    key_command_map: KeyCommandMap, // HashMap<MouseButton, GameCommand>, // KeyCommandMap,
    command_fn_map: HashMap<GameCommand, fn()>
}
// impl Eq for GameCommandFnMap {}


pub fn mock_fn() { println!("mock_fn()"); }

/// This system prints out all keyboard events as they come in
pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("Keyboard event: {:?}", event);
    }
}

/// This system prints out all keyboard events as they come in
pub fn print_mouse_event_system(mut commands: Commands, mut mesh: ResMut<Assets<Mesh>>, mut aterials: ResMut<Assets<StandardMaterial>>, mut mouse_input_events: EventReader<MouseButtonInput>) {
    for event in mouse_input_events.iter() {
        if event.button == MouseButton::Left {
            
            // let position = event.state;
            info!("Mouse event: {:?}", event);
            // create_hex_by_click(position, commands, mesh, materials);
            
        }
        
    }
}

pub fn picking_events(
    mut query: Query<&mut Transform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<PickingEvent>) {
    
        let arc_commands = Arc::new(Mutex::new(commands));
        
        for event in events.iter() {
        println!("picking_events(): This event happened! {:?}", event);
 
        match event {
            PickingEvent::Selection(SelectionEvent::JustSelected(e)) => {
                println!("entid: {:?}", e);

                let entity_transform = query.get_mut(*e);

                if let Ok(mut et) = entity_transform {
                    let color = Color::rgb(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen());
                    let (meh, _) = generate_hex_mesh(0.05, 0.8);
                    let mesh_handle = meshes.add(meh);
                    let mut new_translation = et.translation.clone();
                    new_translation.y += 0.2;
                    add_hex(new_translation, 0.1, color, mesh_handle, Arc::clone(&arc_commands), &mut materials);
                }            
            }
            _ => ()
        };
    }
}



// TODO: Consider events for this.
// TODO: Decouple further = the key-command and command-function map should be independent, right?
/* pub fn init_key_map(mut mapping: ResMut<KeyCommandMap>) {
    mapping.0.insert(MouseButton::Left, GameCommand::CREATE_HEX);
    mapping.0.insert(MouseButton::Right, GameCommand::CREATE_HEX);
    // mapping.0.insert(KeyCode::Up, GameCommand::PLAYER_MOVE);
} */

pub fn init_command_map(key_map: Res<KeyCommandMap>, mut command_map: ResMut<GameCommandFnMap>) {
    command_map.init_key(MouseButton::Left, GameCommand::CREATE_HEX);
    command_map.init_key(MouseButton::Right, GameCommand::CREATE_HEX);
    // Bind commands to functions
    // Q: How to bind to specific component-bound functions? :o Alá actor messages?
    command_map.command_fn_map.insert(GameCommand::PLAYER_MOVE, mock_fn);
    command_map.command_fn_map.insert(GameCommand::CREATE_HEX, mock_fn);
}

impl GameCommandFnMap {

    // Q: Separation of layers. Shouldn't this be Bevy-agnostic?
    fn init_key(&mut self, key_code: MouseButton, game_cmd: GameCommand) {
        self.key_command_map.0.insert(key_code, game_cmd); // key_command_map.clone();
    }

    fn call_cmd_fn(&self, button: MouseButton) {

        let cmd = self.key_command_map.0.get(&button); // .unwrap();

        match cmd {
            Some(cmd) => {
                match self.command_fn_map.get(cmd) {
                    Some(cmd_fn) => {
                        cmd_fn.call(());
                    },
                    None => {
                        println!("GameCommandFnMap::call_cmd_fn(): No fn assigned to the game command.");
                    }
                }
            },
            None => {
                println!("GameCommandFnMap::get_fn(): No command assigned to the key.");
            }
        };
    }
}



// static ref MAP: HashMap<&'static str, fn()> = {        
// t.insert("p", f as fn());
        

pub fn create_hex_by_click(
        mut commands: Commands,
        mut mesh: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {

        // let color = Color::rgb(0.2, 0.6, 0.9);
//        add_hex(position, color, mesh, commands, materials)

    // add_hex(position, color, mesh, commands, materials);
}


pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    key_command_map: Res<KeyCommandMap>,
    command_fn_map: Res<GameCommandFnMap>
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        command_fn_map.call_cmd_fn(MouseButton::Left);
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
        command_fn_map.call_cmd_fn(MouseButton::Right);
    }
}


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
