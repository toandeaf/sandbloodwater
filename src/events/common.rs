use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Event;

pub struct CommonEventsPlugin;

impl Plugin for CommonEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, example)
            .add_event::<CommonTestEvent>();
    }
}

fn example() {}

#[derive(Event)]
struct CommonTestEvent;
