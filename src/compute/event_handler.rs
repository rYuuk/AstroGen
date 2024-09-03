use std::time::Instant;
use bevy::math::{Vec3, Vec4};
use bevy::prelude::{Event, EventReader, EventWriter, Local, ResMut};
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy_easy_compute::prelude::AppComputeWorker;
use crate::compute::asteroid_terrain_generator::{AsteroidComputeWorker, NormalAccumulator};
use crate::RngSeed;
use crate::sphere_mesh::SphereMesh;
use crate::ui_widgets::crater_setting_widget::CraterSettingsChanged;
use crate::ui_widgets::ridge_noise_setting_widget::RidgeNoiseSettingsChanged;
use crate::ui_widgets::simple_noise_setting_widget::SimpleNoiseSettingsChanged;
use crate::utils::PRNG;

#[derive(Event)]
pub struct MeshDataAfterCompute(pub Vec<Vec3>, pub Vec<Vec3>);

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

pub fn receive_data_after_compute(
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    mut data_after_compute: EventWriter<MeshDataAfterCompute>,
) {
    if compute_worker.ready() {
        let raw_vertices: Vec<[f32; 4]> = compute_worker.read_vec("new_vertices");
        let vertex_count = raw_vertices.len();
        let vertices = convert_array4_to_vec3(raw_vertices);
        let raw_normals: Vec<[f32; 4]> = compute_worker.read_vec("normals");
      
        let mut normals = convert_array4_to_vec3(raw_normals);
        data_after_compute.send(MeshDataAfterCompute(vertices, normals));

        // clear normal accumulators from previous run
        compute_worker.write_slice("normal_accumulators", &vec![NormalAccumulator::default(); vertex_count]);
    }
}

fn convert_array4_to_vec3(raw: Vec<[f32; 4]>) -> Vec<Vec3> {
    let mut vec3s = Vec::with_capacity(raw.len());

    for array in raw {
        // Create a Vec3 from the first three elements of the array
        vec3s.push(Vec3::new(array[0], array[1], array[2]));
    }

    vec3s
}