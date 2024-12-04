use crate::compute::triggers::entity_run::EntityTriggerPlugin;
use crate::compute::triggers::player_run::PlayerTriggerPlugin;
use crate::compute::triggers::world_run::WorldTriggerPlugin;
use bevy::app::{App, Plugin};

mod entity_run;
mod player_run;
mod world_run;

pub struct TriggerSystemsPlugin;

impl Plugin for TriggerSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerTriggerPlugin, EntityTriggerPlugin, WorldTriggerPlugin));
    }
}
