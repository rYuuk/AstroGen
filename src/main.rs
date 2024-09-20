use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy::DefaultPlugins;
use bevy::utils::default;
use bevy::winit::WinitSettings;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::asteroid_mesh::AsteroidMeshPlugin;
use crate::compute::ComputePlugin;
use crate::gltf_exporter::GlTFExporter;
use crate::light::LightPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::data::asteroid_settings::AsteroidSettings;
use crate::ui_asteroid_settings::UIAsteroidSettings;

mod asteroid_mesh;
mod gltf_exporter;
mod light;
mod main_camera;
mod data;
mod sphere_mesh;
mod utils;
mod ui_asteroid_settings;
pub mod compute;

#[derive(Resource)]
struct RngSeed(u64);

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AstroGen".into(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((EguiPlugin,
                      FrameTimeDiagnosticsPlugin,
                      AppComputePlugin,
                      ComputePlugin,
                      AsteroidMeshPlugin,
                      UIAsteroidSettings,
                      GlTFExporter,
                      MainCameraPlugin,
                      LightPlugin))
        .insert_resource(RngSeed(2))
        .insert_resource(AsteroidSettings::default())
        .run();
}