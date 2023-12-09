use crate::game::GamePlugin;
use bevy::prelude::*;
use bevy::DefaultPlugins;

mod camera;
mod game;
mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, world!");
}
