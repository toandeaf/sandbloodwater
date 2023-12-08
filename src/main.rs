use bevy::ecs::system::Query;
use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_players)
        .add_systems(Main, (hello_world, iterate_through_players))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

fn add_players(mut commands: Commands) {
    commands.spawn((Player, Name(String::from("Jake!"))));
}

fn iterate_through_players(query: Query<&Name, With<Player>>) {
    for player in query.iter() {
        println!("Player: {}", player.0);
    }
}

fn hello_world() {
    println!("Hello, world!");
}
