mod sphere_mesh;

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::input::mouse::MouseMotion;
use bevy::pbr;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::render::render_resource::ShaderRef;
use bevy_easy_compute::prelude::{AppComputePlugin, AppComputeWorker, AppComputeWorkerBuilder, AppComputeWorkerPlugin, ComputeShader, ComputeWorker};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use sickle_ui::prelude::{SetJustifyContentExt, SetMarginExt, SetWidthExt, Slider, SliderConfig, UiColumnExt, UiRowExt, UiSliderExt};
use sickle_ui::SickleUiPlugin;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};
use sickle_ui::widgets::inputs::slider::SliderBar;
use sphere_mesh::SphereMesh;

#[derive(Component)]
struct Asteroid;

#[derive(Resource)]
struct AsteroidTextures {
    color: Handle<Image>,
    normal: Handle<Image>,
    ao: Handle<Image>,
    roughness: Handle<Image>,
}

#[derive(Component, Debug)]
pub struct SomeSlider;

#[derive(TypePath)]
struct SimpleShader;

impl ComputeShader for SimpleShader {
    fn shader() -> ShaderRef {
        "shaders/compute_asteroid_height.wgsl".into()
    }
}

#[derive(Resource)]
struct SimpleComputeWorker;

impl ComputeWorker for SimpleComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let sphere_mesh = SphereMesh::new(120);
        let len = sphere_mesh.vertices.len();

        let worker = AppComputeWorkerBuilder::new(world)
            .add_staging("vertices", &(sphere_mesh.vertices))
            .add_staging("heights", &vec![0.0; len])
            .add_uniform("numVertices", &(len as u32))
            .add_staging("testValue", &0.)
            .add_pass::<SimpleShader>([512, 1, 1], &["vertices", "heights", "numVertices", "testValue"])
            .one_shot()
            .build();

        world.insert_resource(sphere_mesh);
        worker
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SickleUiPlugin)
        .add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<SimpleComputeWorker>::default())
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_theme_contrast_select)
        .add_systems(Update, rotate_asteroid_system)
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        projection: PerspectiveProjection {
            fov: 45.0f32.to_radians(),
            ..default()
        }
            .into(),
        ..default()
    }, );

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 30.0).looking_at(Vec3::ZERO, Vec3::ZERO),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands.ui_builder(UiRoot).column(|column| {
        column
            .style()
            .width(Val::Percent(100.));
        column.row(|row| {
            row.slider(SliderConfig::horizontal(
                String::from("Distortion"),
                0.,
                20.0,
                0.,
                true,
            )).insert(SomeSlider);
        })
            .style()
            .margin(UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.), Val::ZERO))
            .justify_content(JustifyContent::Center)
            .width(Val::Percent(30.));
    });

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
        sphere_mesh: Res<SphereMesh>,
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
    mesh: Mesh
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 0.5,
                metallic: 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Asteroid
    ));
}

fn handle_theme_contrast_select(
    mut query: Query<&Slider, (With<SomeSlider>, Changed<Slider>)>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
) {
    for mut sliderBar in query.iter_mut() {
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