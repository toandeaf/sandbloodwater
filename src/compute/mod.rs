pub mod response;
mod triggers;

use crate::compute::response::ResponseSystemsPlugin;
use crate::compute::triggers::TriggerSystemsPlugin;
pub use bevy::prelude::*;

pub struct ComputePlugin;

impl Plugin for ComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TriggerSystemsPlugin, ResponseSystemsPlugin));
    }
}
