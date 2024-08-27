use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{EventReader, ResMut};
use bevy_easy_compute::prelude::AppComputeWorker;
use rand::prelude::StdRng;
use rand::SeedableRng;
use crate::crater_setting_widget::CraterSettingsChanged;
use crate::{RngSeed, SimpleComputeWorker};
use crate::ridge_noise_setting_widget::RidgeNoiseSettingsChanged;
use crate::simple_noise_setting_widget::SimpleNoiseSettingsChanged;
use crate::utils::PRNG;

pub struct ComputeDataSender;

impl Plugin for ComputeDataSender {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_crater_settings_changed, on_ridge_settings_changed, on_simple_noise_settings_changed));
    }
}

fn on_crater_settings_changed(
    mut crater_settings_changed: EventReader<CraterSettingsChanged>,
    mut seed: ResMut<RngSeed>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
) {
    for mut ev in crater_settings_changed.read() {
        let crater_settings = &ev.0;
        let craters = crater_settings.get_craters(seed.0);

        let mut craters_centre: Vec<Vec3> = vec![];
        let mut craters_radius: Vec<f32> = vec![];
        let mut craters_floor_height: Vec<f32> = vec![];
        let mut craters_smoothness: Vec<f32> = vec![];

        for crater in craters.clone() {
            craters_centre.push(crater.centre);
            craters_radius.push(crater.radius);
            craters_floor_height.push(crater.floor_height);
            craters_smoothness.push(crater.smoothness);
        }

        compute_worker.write_slice("num_craters", &[crater_settings.get_num_craters() as u32]);
        compute_worker.write_slice("rim_steepness", &[crater_settings.get_rim_steepness()]);
        compute_worker.write_slice("rim_width", &[crater_settings.get_rim_width()]);
        compute_worker.write_slice("craters_centre", &craters_centre);
        compute_worker.write_slice("craters_radius", &craters_radius);
        compute_worker.write_slice("craters_floor_height", &craters_floor_height);
        compute_worker.write_slice("craters_smoothness", &craters_smoothness);

        compute_worker.execute();
    }
}

fn on_simple_noise_settings_changed(
    mut simple_noise_settings_changed: EventReader<SimpleNoiseSettingsChanged>,
    mut seed: ResMut<RngSeed>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
)
{
    for mut ev in simple_noise_settings_changed.read() {
        let ridge_noise_settings = &ev.0;

        let master_seed = seed.0;
        let prng = PRNG {
            seed: master_seed,
            rng: StdRng::seed_from_u64(master_seed),
        };
        let noise_params = ridge_noise_settings.get_noise_params(prng);

        compute_worker.write_slice("noise_params_shape", &noise_params);
        compute_worker.execute();
    }
}

fn on_ridge_settings_changed(
    mut ridge_noise_settings_changed: EventReader<RidgeNoiseSettingsChanged>,
    mut seed: ResMut<RngSeed>,
    mut compute_worker: ResMut<AppComputeWorker<SimpleComputeWorker>>,
)
{
    for mut ev in ridge_noise_settings_changed.read() {
        let ridge_noise_settings = &ev.0;
        let master_seed = seed.0;
        let prng = PRNG {
            seed: master_seed,
            rng: StdRng::seed_from_u64(master_seed),
        };

        let noise_params = ridge_noise_settings.get_noise_params(prng);
        let prefix = "noise_params_";
        let suffix = &ev.1;
        let param_name = &format!("{}{}", prefix, suffix);

        compute_worker.write_slice(param_name, &noise_params);
        compute_worker.execute();
    }
}

