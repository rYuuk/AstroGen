use crate::utils::PRNG;
use bevy::math::Vec3;
use bevy::prelude::Resource;
use bevy::reflect::Reflect;

#[derive(Resource, Default, Debug, Reflect, Clone)]
pub struct RidgeNoiseSettings {
    num_layers: f32,
    lacunarity: f32,
    persistence: f32,
    scale: f32,
    power: f32,
    elevation: f32,
    gain: f32,
    vertical_shift: f32,
    peak_smoothing: f32,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
}

impl RidgeNoiseSettings {
    pub fn get_noise_params(&self, mut prng: PRNG) -> Vec<[f32; 4]> {
        let seeded_offset = Vec3::new(prng.get_value(), prng.get_value(), prng.get_value())
            * prng.get_value()
            * 10000.;

        let noise_params: Vec<[f32; 4]> = vec![
            [
                seeded_offset.x + self.offset_x,
                seeded_offset.y + self.offset_y,
                seeded_offset.z + self.offset_z,
                self.num_layers,
            ],
            [
                self.persistence,
                self.lacunarity,
                self.scale,
                self.elevation,
            ],
            [
                self.power,
                self.gain,
                self.vertical_shift,
                self.peak_smoothing,
            ],
        ];
        noise_params
    }
}
