use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy_easy_compute::prelude::AppComputePlugin;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use sickle_ui::prelude::{ SetJustifyContentExt, SetMarginExt, SetWidthExt, UiColumnExt,};
use sickle_ui::SickleUiPlugin;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};

use compute::asteroid_terrain_generator::AsteroidGeneratorPlugin;
use simple_noise_setting_widget::SimpleNoisePlugin;
use sphere_mesh::SphereMesh;

use crate::compute::event_handler::HeightsAfterCompute;
use crate::crater_setting_widget::{CraterSettingPlugin, CraterSettingWidgetExt};
use crate::light::LightPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::ridge_noise_setting_widget::{RidgeNoisePlugin, RidgeNoiseSettingWidgetExt};
use crate::simple_noise_setting_widget::SimpleNoiseWidgetExt;

mod compute;
mod sphere_mesh;
mod simple_noise_setting_widget;

mod simple_noise_settings;
mod ridge_noise_settings;
mod ridge_noise_setting_widget;

mod utils;
mod crater_settings;
mod crater_setting_widget;
mod main_camera;
mod light;
#[derive(Component)]
struct Asteroid;

#[derive(Component, Debug)]
pub struct TestDestortionSlider;

#[derive(Component, Debug)]
pub struct NumCraterSlider;

#[derive(Component, Debug)]
pub struct CraterMinRadiusSlider;

#[derive(Component, Debug)]
pub struct CraterMaxRadiusSlider;
#[derive(Component, Debug)]
pub struct RimWidthSlider;

#[derive(Component, Debug)]
pub struct RimSteepnessSlider;

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
        .add_plugins((MainCameraPlugin, LightPlugin))
        .add_plugins(CraterSettingPlugin)
        .add_plugins(SimpleNoisePlugin)
        .add_plugins(RidgeNoisePlugin)
        .insert_resource(Msaa::Sample8)
        .insert_resource(RngSeed(2))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_asteroid_system)
        .add_systems(Update, receive_heights_after_compute)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    sphere_mesh: Res<SphereMesh>,
) {
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

    let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sphere_mesh.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sphere_mesh.normals.clone());
    mesh.insert_indices(Indices::U32(sphere_mesh.indices.clone()));

    // let mesh = Mesh::from(sphere_mesh);
    generate_mesh(commands, meshes, materials, mesh);
}

fn receive_heights_after_compute(
    mut height_after_compute: EventReader<HeightsAfterCompute>,
    asteroid_query: Query<Entity, With<Asteroid>>,
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
    
    if let Ok(asteroid_entity) = asteroid_query.get_single() {
        commands.entity(asteroid_entity).despawn();
    }

    let sphere_mesh = SphereMesh::new(400);
    let vertices = sphere_mesh.vertices.clone();

    let mut new_vertices: Vec<Vec3> = vec![];
    for i in 0..vertices.len() {
        new_vertices.push(vertices[i] * heights[i]);
    }

    let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sphere_mesh.normals.clone());
    mesh.insert_indices(Indices::U32(sphere_mesh.indices.clone()));
    generate_mesh(commands, meshes, materials, mesh);
}

fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh: Mesh,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.4, 0.4),
                perceptual_roughness: 1.,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Asteroid
    ));
}

fn rotate_asteroid_system(
    mut query: Query<&mut Transform, With<Asteroid>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        let mut rotation_x = 0.0;
        let mut rotation_y = 0.0;

        for event in mouse_motion_events.read() {
            rotation_x += event.delta.y * 0.005;
            rotation_y += event.delta.x * 0.005;
        }

        for mut transform in query.iter_mut() {
            transform.rotation *= Quat::from_rotation_x(rotation_x);
            transform.rotation *= Quat::from_rotation_y(rotation_y);
        }
    }
}