use bevy::math::{FloatExt, Vec3};
use bevy::prelude::{Reflect, Resource};
use bevy::render::render_resource::ShaderType;
use bevy::utils::HashMap;
use crate::utils::{PRNG};
use bytemuck;
use bytemuck::{Pod, Zeroable};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[repr(C)]
#[derive(ShaderType,Default, Clone, Copy, Pod, Zeroable)]
pub struct Crater {
    pub centre: Vec3,
    pub radius: f32,
    pub floor_height: f32,
    pub smoothness: f32,
}

#[derive(Resource, Default, Debug, Reflect)]
pub struct CraterSettings {
    num_craters: f32,
    crater_size_min: f32,
    crater_size_max: f32,
    rim_steepness: f32,
    rim_width: f32,
    smooth_min: f32,
    smooth_max: f32,
    size_distribution: f32,
}
const CRATER_SEED: u64 = 2;

impl CraterSettings {
    pub fn get_num_craters(&self) -> f32
    {
        self.num_craters
    }

    pub fn get_rim_steepness(&self) -> f32
    {
        self.rim_steepness
    }

    pub fn get_rim_width(&self) -> f32
    {
        self.rim_width
    }

    pub fn get_craters(&self, crater_seed: u64) -> Vec<Crater> {
        // Create craters
        let num_craters = self.num_craters as usize;

        let mut craters = Vec::with_capacity(num_craters);

        let seed = crater_seed + CRATER_SEED;
        let mut prng = PRNG {
            seed,
            rng: StdRng::seed_from_u64(seed),
        };

        for _ in 0..num_craters {
            let t = prng.value_bias_lower(self.size_distribution);
            let size = self.crater_size_min.lerp(self.crater_size_max, t);
            // let floor_height = 0.2;
            let floor_height = -1.2.lerp(-0.2, t + prng.value_bias_lower(0.3));
            // let smoothness = self.smooth_min.lerp(self.smooth_max, 1.0 - t);
            let smoothness = 0.2;

            // Generate a random point on the unit sphere
            let centre = prng.random_on_unit_sphere();
            // println!("{}", smoothness);

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
