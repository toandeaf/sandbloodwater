use bevy::prelude::*;
use bevy::DefaultPlugins;

mod player;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, world!");
}
