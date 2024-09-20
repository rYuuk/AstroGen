use bevy::prelude::Resource;
use crate::data::crater_settings::CraterSettings;
use crate::data::ridge_noise_settings::RidgeNoiseSettings;
use crate::data::simple_noise_settings::SimpleNoiseSettings;

#[derive(Resource, Default)]
pub struct AsteroidSettings
{
    pub crater_settings: CraterSettings,
    pub simple_noise_settings: SimpleNoiseSettings,
    pub ridge_noise_settings: RidgeNoiseSettings,
    pub ridge_noise_settings2: RidgeNoiseSettings,
}