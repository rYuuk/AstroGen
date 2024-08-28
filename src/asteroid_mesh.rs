use bevy::asset::Assets;
use bevy::color::Color;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{App, ButtonInput, Commands, Component, default, EventReader, Mesh, MouseButton, Plugin, Query, Res, ResMut, Transform, Update, With};

pub struct AsteroidMeshPlugin;

impl Plugin for AsteroidMeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, rotate_asteroid_mouse);
    }
}

#[derive(Component)]
pub struct Asteroid;

pub fn generate_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh: Mesh,
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