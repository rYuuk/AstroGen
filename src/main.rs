mod sphere_mesh;

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::input::mouse::MouseMotion;
use bevy::log::tracing_subscriber::filter::FilterExt;
use bevy::pbr;
use bevy::prelude::*;
use bevy::reflect::ReflectKind::Array;
use bevy::render::mesh;
use bevy::render::mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::render::render_resource::ShaderRef;
use bevy_easy_compute::prelude::{AppComputePlugin, AppComputeWorker, AppComputeWorkerBuilder, AppComputeWorkerPlugin, ComputeShader, ComputeWorker};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use sickle_ui::drag_interaction::Draggable;
use sickle_ui::prelude::{SetJustifyContentExt, SetMarginExt, SetWidthExt, Slider, SliderConfig, UiColumnExt, UiRowExt, UiSliderExt};
use sickle_ui::prelude::StylableAttribute::Height;
use sickle_ui::SickleUiPlugin;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};
use sickle_ui::widgets::inputs::slider::SliderBar;
use sphere_mesh::SphereMesh;
// use crate::sphere_mesh::VertexData;

#[derive(Event)]
struct RandomizeAsteroidEvent;

#[derive(Component)]
struct Asteroid;

#[derive(Component)]
struct RandomizeButton;
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

        println!("Build {}", len);

        // Resolution of 4, for example
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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(SickleUiPlugin)
        .add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<SimpleComputeWorker>::default())
        .insert_resource(Msaa::Sample4)
        .add_event::<RandomizeAsteroidEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, randomize_asteroid))
        // .add_systems(Update, update_mesh_vertices)
        .add_systems(Update, handle_theme_contrast_select)
        .add_systems(Update, rotate_asteroid_system)
        .add_systems(Update, test)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    sphere_mesh: Res<SphereMesh>,
    // mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
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

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 3000.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 8.0, 0.0),
    //     ..default()
    // });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 30.0).looking_at(Vec3::ZERO, Vec3::ZERO),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // commands.spawn(DirectionalLight::default());

    let textures = AsteroidTextures {
        color: asset_server.load("textures/asteroid_color.png"),
        normal: asset_server.load("textures/asteroid_normal.png"),
        ao: asset_server.load("textures/asteroid_ao.png"),
        roughness: asset_server.load("textures/asteroid_roughness.png"),
    };
    // Add textures as a resource
    commands.insert_resource(textures);
    // Asteroid
    // let world = commands.world_scope(|world| {
    //     if let Some(textures) = world.get_resource::<AsteroidTextures>() {
    //         // do_something_with_textures(textures);
    //         generate(&mut commands, &mut meshes, &mut materials, textures);
    //     }
    // });

    // generate(&mut commands, &mut meshes, &mut materials, &textures);
    // let compute_shader_handle = asset_server.load("shaders/compute_asteroid_height.wgsl");

    // let sphere_mesh = SphereMesh::new(120); // Resolution of 4, for example
    // let mut new_vertices: Vec<Vec3> = vec![];
    // for vertex in &sphere_mesh.vertices {
    //     // let new_vertex = Vec3::new(vertex.x, 1.0 + (vertex.y * 0.2).sin(), vertex.z);
    //     let new_vertex = vertex.clone();
    //     new_vertices.push(new_vertex);
    // }
    // let mesh = Mesh::from(sphere_mesh);

    // let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sphere_mesh.vertices);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sphere_mesh.normals);
    // mesh.insert_indices(Indices::U32(sphere_mesh.indices));

    // let len = sphere_mesh.vertices.len();

    // compute_worker.write_slice("vertices",&sphere_mesh.vertices.positions);
    // let heights = vec![0.0f32; len];
    // compute_worker.write_slice("height", &heights );
    // compute_worker.write_slice("numVertices",&[len as u32]);

    // let worker: AppComputeWorker<SimpleComputeWorker> = AppComputeWorkerBuilder::new()
    //     .add_storage("vertices", &sphere_mesh.vertices.positions)
    //     .add_staging("heights", &vec![0.0; len])
    //     .add_uniform("numVertices",  &(len as u32))
    //     .add_uniform("testValue", &3.)
    //     .add_pass::<SimpleShader>([4, 1, 1], &["vertices", "heights","numVertices","testValue"])
    //     .one_shot()
    //     .build();
    // 
    // commands.insert_resource(worker);

    println!("Setup");

    let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sphere_mesh.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sphere_mesh.normals.clone());
    mesh.insert_indices(Indices::U32(sphere_mesh.indices.clone()));

    // let mesh = Mesh::from(sphere_mesh);

    // let mesh: Mesh = Mesh::from(sphere_mesh);
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
    // Convert SphereMesh into a Bevy Mesh and add it to the mesh assets

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
}

fn test(mut compute_worker:
        ResMut<AppComputeWorker<SimpleComputeWorker>>,
        asteroid_query: Query<Entity, With<Asteroid>>,
        mut commands: Commands,
        sphere_mesh: Res<SphereMesh>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !compute_worker.ready() {
        return;
    };
    
    let result: Vec<f32> = compute_worker.read_vec("heights");
    let result2: Vec<Vec3> = compute_worker.read_vec("vertices");
    println!("{} {}", result[0], result.len());

    if let Ok(asteroid_entity) = asteroid_query.get_single() {
        commands.entity(asteroid_entity).despawn();
    }

    let vertices = sphere_mesh.vertices.clone();

    let mut new_vertices: Vec<Vec3> = vec![];
    for i in 0..vertices.len() {
        // if result[i] != 1.0
        // {
            // println!("point {} {} {}", i, result[i], result2[i]);
        // }
        new_vertices.push(vertices[i] * result[i]);
    }

    let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sphere_mesh.normals.clone());
    mesh.insert_indices(Indices::U32(sphere_mesh.indices.clone()));

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

    // compute_worker.write_slice::<f32>("heights", &[2., 3., 4., 5.]);
    // for re in result {
    // println!("got {:?}", result[0]);
    // }
}

fn handle_theme_contrast_select(
    mut query: Query<&Slider, (With<SomeSlider>, Changed<Slider>)>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
) {
    for mut sliderBar in query.iter_mut() {
        println!("Called: {}", sliderBar.value());
        compute_worker.write_slice("testValue", &[sliderBar.value()]);
        compute_worker.execute();
    }
}

// fn update_mesh_vertices(
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut query: Query<(&Handle<Mesh>, &mut VertexData), With<Asteroid>>,
//     time: Res<Time>,
// ) {
//     for (mesh_handle, mut vertex_data) in query.iter_mut() {
//         if let Some(mesh) = meshes.get_mut(mesh_handle) {
//             if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
//                 // Update vertex positions
//                 for (i, position) in positions.iter_mut().enumerate() {
//                     let t = time.elapsed_seconds();
//                     vertex_data.positions[i].y += (t * 2.0 + i as f32).sin() * 0.01;
//                     *position = vertex_data.positions[i].to_array();
//                 }
//             }
//         }
//     }
// }

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

    // for vertex in sphere_mesh.vertices.clone() {
    //     gizmos.sphere(vertex,Quat::default(),0.01,color);
    // }
}

fn create_random_asteroid_mesh(size_range: std::ops::Range<f32>) -> Mesh {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::default();
    let size = rng.gen_range(size_range); // Random size within the given range

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();

    let sectors = rng.gen_range(50..=100); // Randomize between 10 and 30
    let stacks = rng.gen_range(60..=100);  // Randomize between 10 and 30
    let noise_scale = rng.gen_range(0.6..=1.0); // Randomize between 0.1 and 1.0
    let displacement_factor = rng.gen_range(0.4..=0.6); // Randomize between 0.1 and 1.0

    // let sectors = 12; // Fewer sectors for more irregularity
    // let stacks = 12; // Fewer stacks for more irregularity
    // let noise_scale = 1.0; // Larger noise scale for more significant distortion
    // let displacement_factor = 0.3; // Control the level of bumpiness

    for i in 0..=stacks {
        let v = i as f32 / stacks as f32;
        let phi = std::f32::consts::PI * v;

        for j in 0..=sectors {
            let u = j as f32 / sectors as f32;
            let theta = std::f32::consts::PI * 2.0 * u;

            // Base spherical coordinates
            let mut x = phi.sin() * theta.cos();
            let mut y = phi.cos();
            let mut z = phi.sin() * theta.sin();

            // Apply Perlin noise to the radius (distance from the origin)
            let noise_value = perlin.get([x as f64 * noise_scale, y as f64 * noise_scale, z as f64 * noise_scale]) as f32;
            let displacement = 1.0 + noise_value * displacement_factor;
            x *= displacement * size;
            y *= displacement * size;
            z *= displacement * size;

            positions.push([x, y, z]);
            normals.push([x, y, z]); // Approximate normals; could be improved
            uvs.push([u, v]);

            if i < stacks && j < sectors {
                let k1 = i * (sectors + 1) + j;
                let k2 = k1 + sectors + 1;

                indices.extend_from_slice(&[k1 as u32, k2 as u32, (k1 + 1) as u32]);
                indices.extend_from_slice(&[k2 as u32, (k2 + 1) as u32, (k1 + 1) as u32]);
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

fn button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RandomizeButton>)>,
    mut randomize_event: EventWriter<RandomizeAsteroidEvent>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            randomize_event.send(RandomizeAsteroidEvent);
        }
    }
}

fn generate(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    textures: &AsteroidTextures,
) {
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(textures.color.clone()),
        normal_map_texture: Some(textures.normal.clone()),
        occlusion_texture: Some(textures.ao.clone()),
        metallic_roughness_texture: Some(textures.roughness.clone()),
        double_sided: false,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(create_random_asteroid_mesh(0.8..2.0)),
            material,
            // material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Asteroid,
    ));
}

fn randomize_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asteroid_query: Query<Entity, With<Asteroid>>,
    mut events: EventReader<RandomizeAsteroidEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<AsteroidTextures>,
) {
    for _ in events.read() {
        if let Ok(asteroid_entity) = asteroid_query.get_single() {
            commands.entity(asteroid_entity).despawn();
        }
        generate(&mut commands, &mut meshes, &mut materials, &textures);
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