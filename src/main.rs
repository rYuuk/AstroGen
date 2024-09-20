use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::winit::WinitSettings;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use compute::asteroid_terrain_generator::AsteroidGeneratorPlugin;

use crate::asteroid_mesh::AsteroidMeshPlugin;
use crate::gltf_exporter::GlTFExporter;
use crate::light::LightPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::settings::asteroid_settings::AsteroidSettings;
use crate::ui_asteroid_settings::UIAsteroidSettings;

mod asteroid_mesh;
mod compute;
mod gltf_exporter;
mod light;
mod main_camera;
mod settings;
mod sphere_mesh;
mod utils;
mod ui_asteroid_settings;

#[derive(Resource)]
struct RngSeed(u64);

#[derive(Component)]
struct ExportButton;

#[derive(Event)]
pub struct ExportButtonClicked;

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
                      AsteroidGeneratorPlugin,
                      AsteroidMeshPlugin,
                      UIAsteroidSettings,
                      GlTFExporter,
                      MainCameraPlugin,
                      LightPlugin))
        .insert_resource(RngSeed(2))
        .add_event::<ExportButtonClicked>()
        .insert_resource(AsteroidSettings::default())
        .run();
}