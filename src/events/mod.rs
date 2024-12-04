pub mod common;
pub mod common_process;
pub mod mutate;
pub mod mutate_process;

use crate::events::common::CommonEventsPlugin;
use crate::events::common_process::CommonEventsProcessingPlugin;
use crate::events::mutate::MutateEventsPlugin;
use crate::events::mutate_process::MutateEventsProcessingPlugin;

use bevy::prelude::*;

pub struct CoreEventsPlugin;

impl Plugin for CoreEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CommonEventsPlugin, CommonEventsProcessingPlugin))
            .add_plugins((MutateEventsPlugin, MutateEventsProcessingPlugin));
    }
}
