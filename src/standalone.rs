use crate::entity_response::EntityResponsePlugin;
use crate::entity_run::EntityRunPlugin;
use crate::event_processing::EventProcessingPlugin;
use crate::events::EventsPlugin;
use crate::player_run::PlayerRunPlugin;
use crate::render::LocalRenderPlugin;
use crate::state_update::StateUpdateSystemsPlugin;
use crate::state_update_events::StateUpdateEventsPlugin;
use crate::world_response::WorldResponsePlugin;
use crate::world_run::WorldRunSystemsPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;

pub struct StandalonePlugin;

impl Plugin for StandalonePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((EventsPlugin, EventProcessingPlugin))
            .add_plugins((StateUpdateEventsPlugin, StateUpdateSystemsPlugin))
            .add_plugins((PlayerRunPlugin, EntityRunPlugin, WorldRunSystemsPlugin))
            .add_plugins((EntityResponsePlugin, WorldResponsePlugin))
            .add_plugins(LocalRenderPlugin)
            .add_systems(Startup, standalone_system);

        // Adding a camera and rendering a sprite
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(CorePipelinePlugin);
        app.add_plugins(SpritePlugin);
        app.add_plugins(RenderPlugin::default());
        app.add_plugins(ImagePlugin::default());

        // The exhaustive set of plugins that come bundled as part of the
        // wider "DefaultPlugins" bundle. Plan is to bring these in as needed.

        // app.add_plugins(TransformPlugin);
        // app.add_plugins(HierarchyPlugin);
        // app.add_plugins(ScenePlugin);
        // app.add_plugins(PipelinedRenderingPlugin);
        // app.add_plugins(TextPlugin);
        // app.add_plugins(UiPlugin);
        // app.add_plugins(PbrPlugin::default());
        // app.add_plugins(GltfPlugin::default());
        // app.add_plugins(AudioPlugin::default());
        // app.add_plugins(GilrsPlugin);
        // app.add_plugins(AnimationPlugin);
        // app.add_plugins(GizmoPlugin);
        // app.add_plugins(StatesPlugin);
    }
}

fn standalone_system() {}