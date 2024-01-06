use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Component)]
pub struct Character {
    name: String
}
