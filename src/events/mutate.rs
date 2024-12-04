use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Event;

pub struct MutateEventsPlugin;

impl Plugin for MutateEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, example)
            .add_event::<MutateTestEvent>();
    }
}

fn example() {}

#[derive(Event)]
struct MutateTestEvent;
