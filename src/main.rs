use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::game::GamePlugin;

mod camera;
mod game;
mod player;
mod world;
mod item;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
