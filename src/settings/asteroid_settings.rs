use bevy::prelude::{default, Resource};
use crate::settings::crater_settings::CraterSettings;
use crate::settings::ridge_noise_settings::RidgeNoiseSettings;
use crate::settings::simple_noise_settings::SimpleNoiseSettings;

#[derive(Resource)]
pub struct AsteroidSettings
{
    pub crater_settings: CraterSettings,
    pub simple_noise_settings: SimpleNoiseSettings,
    pub ridge_noise_settings: RidgeNoiseSettings,
    pub ridge_noise_settings2: RidgeNoiseSettings,
}

impl Default for AsteroidSettings {
    fn default() -> Self {
        AsteroidSettings {
            crater_settings: CraterSettings {
                num_craters: 100.,
                crater_size_min: 0.01,
                crater_size_max: 0.14,
                rim_steepness: 0.13,
                rim_width: 0.61,
                smooth_min: 0.5,
                smooth_max: 0.76,
                size_distribution: 0.05,
            },
            simple_noise_settings: SimpleNoiseSettings {
                num_layers: 3.4,
                lacunarity: 2.,
                persistence: 0.5,
                scale: 0.66,
                elevation: 13.5,
                offset_y: 4.57,
                ..default()
            },
            ridge_noise_settings: RidgeNoiseSettings {
                num_layers: 5.,
                lacunarity: 2.,
                persistence: 0.5,
                scale: 4.44,
                elevation: 0.92,
                power: 0.5,
                gain: 0.5,
                ..default()
            },
            ridge_noise_settings2: RidgeNoiseSettings {
                num_layers: 4.,
                lacunarity: 5.,
                persistence: 0.42,
                scale: 2.97,
                elevation: -3.64,
                gain: 1.,
                peak_smoothing: 1.5,
                ..default()
            },
        }
    }
}
