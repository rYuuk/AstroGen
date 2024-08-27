use bevy::math::Vec3;
use rand::{Rng, SeedableRng};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::StdRng;

pub struct PRNG {
    pub seed: u64,
    pub rng: StdRng
}

impl PRNG {
    pub fn new(seed: u64) -> Self{
        PRNG {
            seed,
            rng: StdRng::seed_from_u64(seed)
        }
    }
    
    pub fn get_value(&mut self) -> f32 {
        let uniform = Uniform::new_inclusive(0.0, 1.0);

        uniform.sample(&mut self.rng)
    }

    pub fn value_bias_lower(&mut self, bias_strength: f32) -> f32 {
        let t = self.get_value();  // Random value [0, 1]

        // Avoid possible division by zero
        if bias_strength == 1.0 {
            return 0.0;
        }

        // Remap strength for nicer input -> output relationship
        let k = (1.0 - bias_strength).clamp(0.0, 1.0);
        let k = k * k * k - 1.0;

        // Apply bias
        ((t + t * k) / (t * k + 1.0)).clamp(0.0, 1.0)
    }

    pub fn random_on_unit_sphere(&mut self) -> Vec3 {
        loop {
            // Generate random points in a cube
            let x: f32 = self.rng.gen_range(-1.0..1.0);
            let y: f32 = self.rng.gen_range(-1.0..1.0);
            let z: f32 = self.rng.gen_range(-1.0..1.0);

            let point = Vec3::new(x, y, z);

            // Check if the point is on the unit sphere
            if point.length_squared() <= 1.0 {
                return point.normalize();
            }
        }
    }
}
