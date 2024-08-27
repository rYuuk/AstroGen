mod sphere_mesh;
mod simple_noise_setting_widget;

mod simple_noise_settings;
mod ridge_noise_settings;
mod ridge_noise_setting_widget;

mod utils;
mod crater_settings;
mod crater_setting_widget;

use std::env::var;
use std::f32::consts::PI;
use bevy::audio::CpalSample;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{vec4, VectorSpace};
use bevy::pbr;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::render::render_resource::{ShaderRef, ShaderSize};
use bevy_easy_compute::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use rand::{Rng, SeedableRng, thread_rng};
use rand::prelude::StdRng;
use rand::rngs::ThreadRng;
use sickle_ui::lerp::Lerp;
use sickle_ui::prelude::{SetHeightExt, SetJustifyContentExt, SetMarginExt, SetWidthExt, Slider, SliderConfig, UiColumnExt, UiRowExt, UiSliderExt};
use sickle_ui::SickleUiPlugin;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};
use sickle_ui::widgets::inputs::slider::SliderBar;
use sphere_mesh::SphereMesh;
use crate::simple_noise_setting_widget::SimpleNoiseWidgetExt;
use simple_noise_setting_widget::SimpleNoisePlugin;
use crate::ridge_noise_setting_widget::{RidgeNoiseSettingWidgetExt, RidgeNoisePlugin};
use crate::crater_setting_widget::{CraterSettingPlugin, CraterSettingWidget, CraterSettingWidgetExt};
use crate::crater_settings::Crater;

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

#[derive(TypePath)]
struct SimpleShader;

#[derive(Resource)]
struct RngSeed(u64);

impl ComputeShader for SimpleShader {
    fn shader() -> ShaderRef {
        "shaders/compute_asteroid_height.wgsl".into()
    }
}

#[derive(Component)]
struct MainCamera;
#[derive(Resource)]
struct SimpleComputeWorker;

struct CraterConfig {
    num_craters: u32,
    rim_steepness: f32,
    rim_width: f32,
    craters: Vec<Crater>,
    vars: Vec<&'static str>,
}

impl CraterConfig {
    pub fn new() -> Self {
        let num_craters = "num_craters";
        let rim_steepness = "rim_steepness";
        let rim_width = "rim_width";
        let craters = "craters";

        CraterConfig {
            num_craters: 0,
            rim_steepness: 0.7,
            rim_width: 1.2,
            craters: vec![Crater::default(); 1000],
            // craters: vec![Crater{
            //     centre:random_on_unit_sphere(), 
            //     radius: 0.03,
            //     floor_height : 0.1,
            //     smoothness : 0.2,
            // }, Crater{
            //     centre:random_on_unit_sphere(),
            //     radius: 0.07,
            //     floor_height : 0.2,
            //     smoothness : 0.3,
            // },
            // ],
            vars: vec![num_craters, rim_steepness, rim_width, "craters_centre", "craters_radius", "craters_floor_height", "craters_smoothness"],
        }
    }
}

pub fn random_on_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        // Generate random points in a cube
        let x: f32 = rng.gen_range(-1.0..1.0);
        let y: f32 = rng.gen_range(-1.0..1.0);
        let z: f32 = rng.gen_range(-1.0..1.0);

        let point = Vec3::new(x, y, z);

        // Check if the point is on the unit sphere
        if point.length_squared() <= 1.0 {
            // let a = point.normalize();

            return point.normalize();
        }
    }
}

impl ComputeWorker for SimpleComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let sphere_mesh = SphereMesh::new(400);
        let len = sphere_mesh.vertices.len();

        let mut vars: Vec<&str> = vec!["vertices", "heights", "numVertices", "testValue", "noise_params_shape", "noise_params_ridge", "noise_params_ridge2"];

        let mut crater_config = CraterConfig::new();
        vars.append(&mut crater_config.vars);

        let noise_params: Vec<[f32; 4]> = vec![
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];

        let mut worker = AppComputeWorkerBuilder::new(world)
            .add_staging(vars[0], &(sphere_mesh.vertices))
            .add_staging(vars[1], &vec![0.0; len])
            .add_uniform(vars[2], &(len as u32))
            .add_uniform(vars[3], &0.)
            .add_storage(vars[4], &noise_params)
            .add_storage(vars[5], &noise_params)
            .add_storage(vars[6], &noise_params)
            .add_uniform(vars[7], &crater_config.num_craters)
            .add_uniform(vars[8], &crater_config.rim_steepness)
            .add_uniform(vars[9], &crater_config.rim_width)
            .add_storage("craters_centre", &&vec![Vec3::ZERO; 1000])
            .add_storage("craters_radius", &&vec![0.0; 1000])
            .add_storage("craters_floor_height", &&vec![0.0; 1000])
            .add_storage("craters_smoothness", &&vec![0.0; 1000])
            .add_pass::<SimpleShader>([1024, 1, 1], &vars)
            .one_shot()
            .build();

        world.insert_resource(sphere_mesh);
        worker
    }
}

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(SickleUiPlugin)
        .add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<SimpleComputeWorker>::default())
        .add_plugins(CraterSettingPlugin)
        .add_plugins(SimpleNoisePlugin)
        .add_plugins(RidgeNoisePlugin)
        .insert_resource(Msaa::Sample8)
        .insert_resource(RngSeed(2))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(Startup, setup)
        .add_systems(Update, handle_theme_contrast_select)
        .add_systems(Update, (rotate_asteroid_system, zoom_camera))
        .add_systems(Update, test)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sphere_mesh: Res<SphereMesh>,
) {
    // Camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 9.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        projection: PerspectiveProjection {
            fov: 45.0f32.to_radians(),
            ..default()
        }
            .into(),
        ..default()
    },
                    MainCamera),
    );


    commands.spawn(DirectionalLightBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        directional_light: DirectionalLight {
            shadows_enabled: true,
            shadow_depth_bias: 0.005,
            shadow_normal_bias: 0.002,
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            color: Color::WHITE,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .into(),
        ..default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Circle::new(4.0)),
    //     material: materials.add(Color::WHITE),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });

    commands.ui_builder(UiRoot).column(|row| {
        row.slider(SliderConfig::horizontal(
            String::from("TestDistortion"),
            0.,
            20.0,
            0.,
            true,
        )).insert(TestDestortionSlider);

        row.create_crater_setting_widget(|x| {});
        row.create_simple_noise_setting_widget(|x| {});
        row.create_ridge_noise_setting_widget(|y| {}, "ridge".to_string());
        row.create_ridge_noise_setting_widget(|y| {}, "ridge2".to_string());
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

fn test(mut compute_worker:
        ResMut<AppComputeWorker<SimpleComputeWorker>>,
        asteroid_query: Query<Entity, With<Asteroid>>,
        mut commands: Commands,
        // sphere_mesh: Res<SphereMesh>,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
) {
    if !compute_worker.ready() {
        return;
    };

    let result: Vec<f32> = compute_worker.read_vec("heights");

    if let Ok(asteroid_entity) = asteroid_query.get_single() {
        commands.entity(asteroid_entity).despawn();
    }

    let sphere_mesh = SphereMesh::new(400);
    let vertices = sphere_mesh.vertices.clone();

    let mut new_vertices: Vec<Vec3> = vec![];
    for i in 0..vertices.len() {
        new_vertices.push(vertices[i] * result[i]);
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

fn handle_theme_contrast_select(
    mut testDestortionQuery: Query<&Slider, (With<TestDestortionSlider>, Changed<Slider>)>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
) {
    for mut sliderBar in testDestortionQuery.iter_mut() {
        compute_worker.write_slice("testValue", &[sliderBar.value()]);
        compute_worker.execute();
    }
}

fn gizmo_update(mut gizmos: Gizmos, sphere_mesh: Res<SphereMesh>) {
    let center = Vec3::new(0.0, 0.0, 0.0);
    let radius = 1.0;

    let color = Color::srgb(1.0, 0.0, 0.0);

    for i in (0..sphere_mesh.indices.len()).step_by(3) {
        let v1 = center + sphere_mesh.vertices[sphere_mesh.indices[i] as usize] * radius;
        let v2 = center + sphere_mesh.vertices[sphere_mesh.indices[i + 1] as usize] * radius;
        let v3 = center + sphere_mesh.vertices[sphere_mesh.indices[i + 2] as usize] * radius;

        gizmos.line(v1, v2, color);
        gizmos.line(v2, v3, color);
        gizmos.line(v3, v1, color);
    }
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

fn zoom_camera(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let scroll = scroll_evr.read().map(|ev| ev.y).sum::<f32>();
    if scroll == 0.0 {
        return;
    }

    for mut transform in camera_query.iter_mut() {
        let mut translation = transform.translation;
        translation.z -= scroll * 0.5; // Adjust this value to change zoom speed
        translation.z = translation.z.max(2.0).min(20.0); // Limit zoom range
        transform.translation = translation;
    }
}