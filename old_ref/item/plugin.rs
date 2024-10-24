use bevy::prelude::{App, Plugin, Startup};

use crate::item::system::initialise_item;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialise_item);
    }
}
