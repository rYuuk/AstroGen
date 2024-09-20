use bevy::asset::{Asset, Assets, Handle};
use bevy::color::Color;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{Material, MaterialMeshBundle, MaterialPlugin, StandardMaterial};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use crate::data::compute_data::MeshDataAfterCompute;
use crate::light::LightDirection;
use crate::sphere_mesh::SphereMesh;

pub struct AsteroidMeshPlugin;

impl Plugin for AsteroidMeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<TriplanarMaterial>::default())
            .observe(generate_mesh_from_new_vertices)
            .add_systems(Update, rotate_asteroid_mouse);
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
    rot: Quat,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.4, 0.4),
                perceptual_roughness: 0.9,
                ..default()
            }),

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
    trigger: Trigger<MeshDataAfterCompute>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    sphere_mesh: ResMut<SphereMesh>,
    light_direction: Res<LightDirection>,
) {
    let ev = trigger.event();
    let new_vertices = ev.0.clone();
    let normals = ev.1.clone();

    if new_vertices.is_empty()
    {
        return;
    }

    let mut rot = Quat::default();

    if let Ok(asteroid_entity) = asteroid_query.get_single() {
        rot = asteroid_entity.1.rotation;
        commands.entity(asteroid_entity.0).despawn();
    }

    let mesh = generate_mesh(new_vertices, sphere_mesh.indices.clone(), normals);
    render_generated_asteroid(commands, mesh, materials, meshes, rot);
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
