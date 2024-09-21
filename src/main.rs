use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy::DefaultPlugins;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::asteroid_mesh_builder::AsteroidMeshBuilderPlugin;
use crate::compute::ComputePlugin;
use crate::gltf_exporter::GlTFExporter;
use crate::light::LightPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::ui_asteroid_settings::UIAsteroidSettings;

mod asteroid_mesh_builder;
mod gltf_exporter;
mod light;
mod main_camera;
mod settings;
mod sphere_mesh;
mod utils;
mod ui_asteroid_settings;
pub mod compute;
pub mod compute_shaders;
mod compute_events;

#[derive(Resource)]
struct RngSeed(u64);

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AstroGen".into(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }),
                      EguiPlugin,
                      FrameTimeDiagnosticsPlugin,
                      AppComputePlugin,
                      ComputePlugin,
                      AsteroidMeshBuilderPlugin,
                      GlTFExporter,
                      MainCameraPlugin,
                      LightPlugin,
                      UIAsteroidSettings,
        ))
        .insert_resource(RngSeed(2))
        .run();
}
