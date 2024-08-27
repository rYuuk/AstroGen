use bevy::prelude::{Event, EventReader, EventWriter, ResMut};
use bevy_easy_compute::prelude::AppComputeWorker;
use crate::compute::asteroid_terrain_generator::AsteroidComputeWorker;
use crate::RngSeed;
use crate::crater_setting_widget::CraterSettingsChanged;
use crate::ridge_noise_setting_widget::RidgeNoiseSettingsChanged;
use crate::simple_noise_setting_widget::SimpleNoiseSettingsChanged;
use crate::utils::PRNG;

#[derive(Event)]
pub struct HeightsAfterCompute(pub Vec<f32>);

pub fn on_crater_settings_changed(
    mut crater_settings_changed: EventReader<CraterSettingsChanged>,
    seed: ResMut<RngSeed>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
) {
    for ev in crater_settings_changed.read() {
        let crater_settings = &ev.0;
        let craters = crater_settings.get_craters(seed.0);

        let (mut centres, mut radii, mut floor_heights, mut smoothness) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for crater in craters.iter() {
            centres.push(crater.centre);
            radii.push(crater.radius);
            floor_heights.push(crater.floor_height);
            smoothness.push(crater.smoothness);
        }

        compute_worker.write_slice("num_craters", &[crater_settings.get_num_craters() as u32]);
        compute_worker.write_slice("rim_steepness", &[crater_settings.get_rim_steepness()]);
        compute_worker.write_slice("rim_width", &[crater_settings.get_rim_width()]);
        compute_worker.write_slice("craters_centre", &centres);
        compute_worker.write_slice("craters_radius", &radii);
        compute_worker.write_slice("craters_floor_height", &floor_heights);
        compute_worker.write_slice("craters_smoothness", &smoothness);

        compute_worker.execute();
    }
}

pub fn on_simple_noise_settings_changed(
    mut simple_noise_settings_changed: EventReader<SimpleNoiseSettingsChanged>,
    seed: ResMut<RngSeed>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
) {
    for ev in simple_noise_settings_changed.read() {
        let simple_noise_settings = &ev.0;
        let prng = PRNG::new(seed.0);
        let noise_params = simple_noise_settings.get_noise_params(prng);

        compute_worker.write_slice("noise_params_shape", &noise_params);
        compute_worker.execute();
    }
}

pub fn on_ridge_settings_changed(
    mut ridge_noise_settings_changed: EventReader<RidgeNoiseSettingsChanged>,
    seed: ResMut<RngSeed>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
) {
    for ev in ridge_noise_settings_changed.read() {
        let ridge_noise_settings = &ev.0;
        let prng = PRNG::new(seed.0);
        let noise_params = ridge_noise_settings.get_noise_params(prng);
        let param_name = format!("noise_params_{}", ev.1);

        compute_worker.write_slice(&param_name, &noise_params);
        compute_worker.execute();
    }
}

pub fn receive_heights_after_compute(
    compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    mut heights_after_compute: EventWriter<HeightsAfterCompute>,
) {
    if compute_worker.ready() {
        let result: Vec<f32> = compute_worker.read_vec("heights");
        heights_after_compute.send(HeightsAfterCompute(result));
    }
}
