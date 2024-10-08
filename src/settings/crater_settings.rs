use crate::utils::PRNG;
use bevy::math::{FloatExt, Vec3};
use bevy::prelude::{Reflect, Resource};
use bevy::render::render_resource::ShaderType;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[repr(C)]
#[derive(ShaderType, Clone, Default, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Crater {
    pub centre: Vec3,
    pub radius: f32,
    pub floor_height: f32,
    pub smoothness: f32,
}

#[derive(Resource, Default, Debug, Reflect, Clone)]
pub struct CraterSettings {
    pub num_craters: f32,
    pub crater_size_min: f32,
    pub crater_size_max: f32,
    pub rim_steepness: f32,
    pub rim_width: f32,
    pub smooth_min: f32,
    pub smooth_max: f32,
    pub size_distribution: f32,
}


const CRATER_SEED: u64 = 2;
pub const MAX_CRATER: usize = 2000;

impl CraterSettings {
    pub fn get_rim_steepness(&self) -> f32 {
        self.rim_steepness
    }

    pub fn get_rim_width(&self) -> f32 {
        self.rim_width
    }

    pub fn get_craters(&self, crater_seed: u64) -> Vec<Crater> {
        // Create craters
        let num_craters = self.num_craters as usize;

        let mut craters = Vec::with_capacity(num_craters);

        let seed = crater_seed + CRATER_SEED;
        let mut prng = PRNG {
            rng: StdRng::seed_from_u64(seed),
        };

        for _ in 0..num_craters {
            let t = prng.value_bias_lower(self.size_distribution);
            let size = self.crater_size_min.lerp(self.crater_size_max, t);
            let floor_height = -1.2.lerp(-0.2, t + prng.value_bias_lower(0.3));
            let smoothness = self.smooth_min.lerp(self.smooth_max, 1.0 - t);

            // Generate a random point on the unit sphere
            let centre = prng.random_on_unit_sphere();

            craters.push(Crater {
                centre,
                radius: size,
                floor_height,
                smoothness,
            });
        }

        craters
    }
}
