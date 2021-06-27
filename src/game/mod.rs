pub mod components;
pub mod board;
pub mod bonus;
pub mod player;
pub mod account;

use bevy::prelude::{ Color, Entity, Handle, Scene, Vec3 };

pub struct Cell {
    pub height: f32,
}

#[derive(Default)]
pub struct BoardParams {
    pub size_x: usize,
    pub size_y: usize
}

pub struct BoardColors {
    // pub colors: Vec<Color>
    pub colors: [bevy::prelude::Color; 3]
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub i: usize,
    pub j: usize,
}

#[derive(Default)]
pub struct Bonus {
    pub entity: Option<Entity>,
    pub i: usize,
    pub j: usize,
    pub handle: Handle<Scene>,
}

#[derive(Default)]
pub struct Game {
    pub board: Vec<Vec<Cell>>,
    pub player: Player,
    pub bonus: Bonus,
    pub score: i32,
    pub cake_eaten: u32,
    pub camera_should_focus: Vec3,
    pub camera_is_focus: Vec3,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    FinishedLoading,
    Playing,
    GameOver,
}