use bevy::asset::Assets;
use bevy::color::Color;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{App, ButtonInput, Commands, Component, default, Entity, EventReader, Mesh, MouseButton, Plugin, Query, Res, ResMut, Transform, Update, With};
use bevy::render::mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use crate::compute::event_handler::HeightsAfterCompute;
use crate::sphere_mesh::SphereMesh;

pub struct AsteroidMeshPlugin;

impl Plugin for AsteroidMeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (generate_mesh_from_new_heights, rotate_asteroid_mouse));
    }
}

#[derive(Component)]
pub struct Asteroid;

pub fn render_generated_asteroid(
    mut commands: Commands,
    mut mesh: Mesh,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    rot: Quat,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.4, 0.4),
                // perceptual_roughness: 1.,
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
fn generate_mesh_from_new_heights(
    mut height_after_compute: EventReader<HeightsAfterCompute>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut sphere_mesh: ResMut<SphereMesh>,
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

    // let vertices = sphere_mesh.vertices.clone();

    let mut new_vertices: Vec<Vec3> = vec![];
    for i in 0..sphere_mesh.vertices.len() {
        new_vertices.push(sphere_mesh.vertices[i] * heights[i]);
    }

    let mesh = generate_mesh(sphere_mesh.vertices.clone(), heights, sphere_mesh.indices.clone());
    // sphere_mesh.vertices = new_vertices;
    render_generated_asteroid(commands, mesh, materials, meshes, rot);
}

fn generate_mesh(vertices: Vec<Vec3>, heights: Vec<f32>, indices: Vec<u32>) -> Mesh {
    let mut new_vertices: Vec<Vec3> = vec![];
    for i in 0..vertices.len() {
        new_vertices.push(vertices[i] * heights[i]);
    }
    let normals = recalculate_normals(&new_vertices, &indices);
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn recalculate_normals(vertices: &Vec<Vec3>, indices: &Vec<u32>) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; vertices.len()];

    for triangle in indices.chunks(3) {
        let i0 = triangle[0] as usize;
        let i1 = triangle[1] as usize;
        let i2 = triangle[2] as usize;

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];

        let normal = (v1 - v0).cross(v2 - v0).normalize();
        // Assign the same normal to each vertex in the triangle
        normals[i0] = normal;
        normals[i1] = normal;
        normals[i2] = normal;
    }

    // Optionally, make sure sharp edges are preserved by not normalizing across entire mesh:
    for i in 0..normals.len() {
        if normals[i].length() > 0.001 {
            normals[i] = normals[i].normalize();
        } else {
            // Handle cases where the normal might be too small
            normals[i] = Vec3::new(0.0, 1.0, 0.0); // Default direction, adjust as needed
        }
    }
    normals
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
