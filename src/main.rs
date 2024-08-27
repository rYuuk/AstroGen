use bevy::prelude::*;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use sickle_ui::prelude::{SetJustifyContentExt, SetMarginExt, SetWidthExt, UiColumnExt};
use sickle_ui::SickleUiPlugin;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};

use compute::asteroid_terrain_generator::AsteroidGeneratorPlugin;
use sphere_mesh::SphereMesh;
use crate::asteroid_mesh::{Asteroid, AsteroidMeshPlugin, generate_mesh};
use crate::compute::event_handler::HeightsAfterCompute;
use crate::light::LightPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::ui_widgets::crater_setting_widget::{CraterSettingPlugin, CraterSettingWidgetExt};
use crate::ui_widgets::ridge_noise_setting_widget::{RidgeNoisePlugin, RidgeNoiseSettingWidgetExt};
use crate::ui_widgets::simple_noise_setting_widget::{SimpleNoisePlugin, SimpleNoiseWidgetExt};

mod compute;
mod settings;
mod ui_widgets;
mod sphere_mesh;
mod asteroid_mesh;
mod utils;
mod main_camera;
mod light;

#[derive(Resource)]
struct RngSeed(u64);

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(SickleUiPlugin)
        .add_plugins(AppComputePlugin)
        .add_plugins(AsteroidGeneratorPlugin)
        .add_plugins(AsteroidMeshPlugin)
        .add_plugins((MainCameraPlugin, LightPlugin))
        .add_plugins(CraterSettingPlugin)
        .add_plugins(SimpleNoisePlugin)
        .add_plugins(RidgeNoisePlugin)
        .insert_resource(Msaa::Sample8)
        .insert_resource(RngSeed(2))
        .add_systems(Startup, setup)
        .add_systems(Update, generate_mesh_from_new_heights)
        .run();
}

fn setup(mut commands: Commands, ) {
    commands.ui_builder(UiRoot).column(|row| {
        row.create_crater_setting_widget(|_x| {});
        row.create_simple_noise_setting_widget(|_x| {});
        row.create_ridge_noise_setting_widget(|_y| {}, "ridge".to_string());
        row.create_ridge_noise_setting_widget(|_y| {}, "ridge2".to_string());
    })
        .style()
        .margin(UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(1.), Val::ZERO))
        .justify_content(JustifyContent::FlexStart)
        .width(Val::Percent(30.));
}

fn generate_mesh_from_new_heights(
    mut height_after_compute: EventReader<HeightsAfterCompute>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut heights: Vec<f32> = vec![];

    for ev in height_after_compute.read() {
        heights = ev.0.clone();
    }

    if heights.len() == 0
    {
        return;
    }
    
    let mut rot = Quat::default();
    
    if let Ok(asteroid_entity) = asteroid_query.get_single() {
        rot = asteroid_entity.1.rotation;
        commands.entity(asteroid_entity.0).despawn();
    }

    let mut sphere_mesh = SphereMesh::new(400);
    let vertices = sphere_mesh.vertices.clone();

    let mut new_vertices: Vec<Vec3> = vec![];
    for i in 0..vertices.len() {
        new_vertices.push(vertices[i] * heights[i]);
    }

    sphere_mesh.vertices = new_vertices;
    generate_mesh(commands, meshes, materials, sphere_mesh.into(),rot);
}


