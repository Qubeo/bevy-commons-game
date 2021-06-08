use bevy::prelude::{ Entity, Handle, Scene, Vec3 };

pub struct Cell {
    pub height: f32,
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
    Playing,
    GameOver,
}