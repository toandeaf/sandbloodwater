use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::game::GamePlugin;

mod camera;
mod game;
mod item;
mod player;
mod world;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
