use std::f32::consts::PI;

use bevy::app::App;
use bevy::color::Color;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{
    CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, DirectionalLightShadowMap,
    light_consts,
};
use bevy::prelude::{Commands, default, Plugin, Startup, Transform};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, initialize_camera);
    }
}

fn initialize_camera(mut commands: Commands) {
    let transform = Transform {
        translation: Vec3::new(0.0, 2.0, 0.0),
        rotation: Quat::from_rotation_x(-PI / 4.),
        ..default()
    };

    commands.spawn(DirectionalLightBundle {
        transform,
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            color: Color::WHITE,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });
}
