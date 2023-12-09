use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::game::GamePlugin;

mod camera;
mod game;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, world!");
}
