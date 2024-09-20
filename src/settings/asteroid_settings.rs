use bevy::prelude::Resource;
use crate::settings::crater_settings::CraterSettings;
use crate::settings::ridge_noise_settings::RidgeNoiseSettings;
use crate::settings::simple_noise_settings::SimpleNoiseSettings;

#[derive(Resource, Default)]
pub struct AsteroidSettings
{
    pub crater_settings: CraterSettings,
    pub simple_noise_settings: SimpleNoiseSettings,
    pub ridge_noise_settings: RidgeNoiseSettings,
    pub ridge_noise_settings2: RidgeNoiseSettings,
}