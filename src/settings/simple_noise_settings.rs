use crate::utils::PRNG;
use bevy::math::Vec3;
use bevy::prelude::Resource;
use bevy::reflect::Reflect;

#[derive(Resource, Default, Debug, Reflect, Clone)]
pub struct SimpleNoiseSettings {
    pub num_layers: f32,
    pub lacunarity: f32,
    pub persistence: f32,
    pub scale: f32,
    pub elevation: f32,
    pub vertical_shift: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
}

impl SimpleNoiseSettings {
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
            [self.vertical_shift, 0.0, 0.0, 0.0],
        ];
        noise_params
    }
}
