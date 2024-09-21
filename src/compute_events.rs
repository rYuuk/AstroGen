use bevy::math::Vec3;
use bevy::prelude::Event;
use crate::settings::crater_settings::CraterSettings;
use crate::settings::ridge_noise_settings::RidgeNoiseSettings;
use crate::settings::simple_noise_settings::SimpleNoiseSettings;

#[derive(Event)]
pub struct MeshDataAfterCompute(pub Vec<Vec3>, pub Vec<Vec3>);

#[derive(Event)]
pub struct CraterSettingsChanged(pub CraterSettings);

#[derive(Event)]
pub struct SimpleNoiseSettingsChanged(pub SimpleNoiseSettings);

#[derive(Event)]
pub struct RidgeNoiseSettingsChanged(pub RidgeNoiseSettings, pub String);