use std::time::Instant;
use bevy::asset::{Asset, Assets, AssetServer, Handle};
use bevy::color::Color;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{Material, MaterialMeshBundle, MaterialPlugin, PbrBundle, StandardMaterial};
use bevy::prelude::{AlphaMode, App, ButtonInput, Commands, Component, default, Entity, EventReader, Gizmos, Image, Mesh, MouseButton, Plugin, Query, Res, ResMut, Transform, TypePath, Update, With};
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use crate::compute::event_handler::MeshDataAfterCompute;
use crate::ExportButtonClicked;
use crate::light::LightDirection;
use crate::sphere_mesh::SphereMesh;

pub struct AsteroidMeshPlugin;

impl Plugin for AsteroidMeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<TriplanarMaterial>::default())
            .add_systems(Update, (generate_mesh_from_new_vertices, rotate_asteroid_mouse));
    }
}

#[derive(Component)]
pub struct Asteroid;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct TriplanarMaterial {
    #[uniform(0)]
    pub scale: f32,
    #[uniform(1)]
    pub blend_sharpness: f32,
    #[texture(2)]
    #[sampler(3)]
    pub main_texture: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    pub normal_map: Option<Handle<Image>>,
    #[uniform(6)]
    pub light_direction: Vec3,
}

impl Material for TriplanarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/triplanar_fragment.wgsl".into()
    }
}

pub fn render_generated_asteroid(
    mut commands: Commands,
    mesh: Mesh,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    rot: Quat,
    light_direction: Res<LightDirection>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.4, 0.4),
                perceptual_roughness: 0.9,
                ..default()
            }),
            // material: materials.add(TriplanarMaterial {
            //     scale: 1.0,
            //     blend_sharpness: 7.0,
            //     main_texture: Some(asset_server.load("textures/MoonNoise.jpg")),
            //     normal_map: Some(asset_server.load("textures/Craters.jpg")),
            //     light_direction: light_direction.direction,
            // }),
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: rot,
                ..default()
            },
            ..default()
        },
        Asteroid
    ));
}
fn generate_mesh_from_new_vertices(
    mut height_after_compute: EventReader<MeshDataAfterCompute>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    sphere_mesh: ResMut<SphereMesh>,
    asset_server: Res<AssetServer>,
    light_direction: Res<LightDirection>,
) {
    let mut new_vertices: Vec<Vec3> = vec![];
    let mut normals: Vec<Vec3> = vec![];

    for ev in height_after_compute.read() {
        new_vertices = ev.0.clone();
        normals = ev.1.clone();
    }

    if new_vertices.len() == 0
    {
        return;
    }

    let mut rot = Quat::default();

    if let Ok(asteroid_entity) = asteroid_query.get_single() {
        rot = asteroid_entity.1.rotation;
        commands.entity(asteroid_entity.0).despawn();
    }

    let mesh = generate_mesh(new_vertices, sphere_mesh.indices.clone(), normals);
    render_generated_asteroid(commands, mesh, materials, meshes, asset_server, rot, light_direction);
}

fn generate_mesh(vertices: Vec<Vec3>, indices: Vec<u32>, normals: Vec<Vec3>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn rotate_asteroid_mouse(
    mut query: Query<&mut Transform, With<Asteroid>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
)
{
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
