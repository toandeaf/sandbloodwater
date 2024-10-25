mod client;
mod compute;
mod events;
mod local;
mod player;
mod remote;
mod server;
mod standalone;

use crate::events::CoreEventsPlugin;
use crate::standalone::StandalonePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(CoreEventsPlugin)
        // The below plugin dictates the execution mode: client, server or standalone (both client and server)
        .add_plugins(StandalonePlugin)
        .run();

    // The exhaustive set of plugins that come bundled as part of the
    // wider "DefaultPlugins" bundle. Plan is to bring these in as needed
    // and allocate them to the appropriate plugin.

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
