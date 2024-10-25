use crate::player::PlayerPlugin;
use bevy::a11y::AccessibilityPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;
use bevy::window::PrimaryWindow;
use bevy::winit::{WakeUp, WinitPlugin};

const TOP_LAYER_Z_INDEX: f32 = 1.;

pub struct LocalRenderPlugin;

impl Plugin for LocalRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, (render_plugin, camera_setup));

        // Window shit
        app.add_plugins(WindowPlugin::default());
        app.add_plugins(WinitPlugin::<WakeUp>::default());
        app.add_plugins(AccessibilityPlugin);

        // Logs
        app.add_plugins(LogPlugin::default());

        // Input
        app.add_plugins(InputPlugin);

        // Adding a camera and rendering a sprite
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin::default());
        app.add_plugins(ImagePlugin::default());
        app.add_plugins(CorePipelinePlugin);
        app.add_plugins(SpritePlugin);

        // Adding player
        app.add_plugins(PlayerPlugin);
    }
}

fn render_plugin() {
    println!("Initialising local render plugin.");
}

fn camera_setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Middle of window
    let x_pos = window.width() / 2.;
    let y_pos = window.height() / 2.;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(x_pos, y_pos, TOP_LAYER_Z_INDEX),
        ..default()
    });

    let sprite_bundle = SpriteBundle {
        transform: Transform::from_xyz(x_pos, y_pos, 0.),
        sprite: Sprite {
            color: Color::linear_rgb(255., 0., 0.),
            custom_size: Some(Vec2::new(50.0, 50.)),
            ..default()
        },
        ..default()
    };
    commands.spawn(sprite_bundle);
}
