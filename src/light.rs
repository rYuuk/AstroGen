use std::f32::consts::PI;
use bevy::app::App;
use bevy::color::Color;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, DirectionalLightShadowMap, light_consts};
use bevy::prelude::{Commands, default, Plugin, Startup, Transform};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, initialize_camera);
    }
}

fn initialize_camera(
    mut commands: Commands
) {
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
}