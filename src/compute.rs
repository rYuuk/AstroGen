use bevy::math::Vec3;
use bevy::prelude::{App, Commands, Plugin, Res, ResMut, Resource, Trigger, Update, World};
use bevy::render::render_resource::ShaderType;
use bevy_easy_compute::prelude::{
    AppComputeWorker, AppComputeWorkerBuilder, AppComputeWorkerPlugin, ComputeWorker,
};
use bytemuck::{Pod, Zeroable};
use crate::compute_shaders::{AsteroidHeightComputeShader, NormalComputeShader, NormalizeNormalComputeShader};
use crate::compute_events::{CraterSettingsChanged, MeshDataAfterCompute, PerturbStrengthChanged, RidgeNoiseSettingsChanged, SimpleNoiseSettingsChanged};
use crate::RngSeed;
use crate::settings::asteroid_settings::AsteroidSettings;
use crate::settings::crater_settings::{Crater, MAX_CRATER};
use crate::sphere_mesh::SphereMesh;
use crate::utils::PRNG;

pub struct ComputePlugin;
#[derive(Resource)]
pub struct AsteroidComputeWorker;

#[repr(C)]
#[derive(ShaderType, Clone, Default, Copy, Pod, Zeroable)]
pub struct NormalAccumulator {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Plugin for ComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AppComputeWorkerPlugin::<AsteroidComputeWorker>::default())
            .insert_resource(AsteroidSettings::default())
            .add_event::<MeshDataAfterCompute>()
            .add_event::<PerturbStrengthChanged>()
            .add_event::<CraterSettingsChanged>()
            .add_event::<SimpleNoiseSettingsChanged>()
            .add_event::<RidgeNoiseSettingsChanged>()
            .observe(send_perturb_strength_data)
            .observe(send_crater_settings_data)
            .observe(send_simple_noise_settings_data)
            .observe(send_ridge_noise_settings_data)
            .add_systems(Update, receive_data_after_compute);
    }
}

impl ComputeWorker for AsteroidComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        const SPHERE_RESOLUTION: usize = 400;
        const WORKGROUP_SIZE: u32 = 64; // This should match @workgroup_size in the shader
        const NUM_NOISE_PARAMS: usize = 3;

        let sphere_mesh = SphereMesh::new(SPHERE_RESOLUTION);
        let vertex_count = sphere_mesh.vertices.len();
        let noise_params: Vec<[f32; 4]> = vec![[0.0; 4]; NUM_NOISE_PARAMS];
        let indices_len = sphere_mesh.indices.len();
        let num_triangles = indices_len / 3;

        let num_workgroups = (vertex_count + WORKGROUP_SIZE as usize - 1) / WORKGROUP_SIZE as usize;
        let num_workgroups_normal = (num_triangles + WORKGROUP_SIZE as usize - 1) / WORKGROUP_SIZE as usize;

        let worker = AppComputeWorkerBuilder::new(world)
            .add_storage("vertices", &sphere_mesh.vertices)
            .add_staging("normals", &vec![Vec3::ZERO; vertex_count])
            .add_storage("indices", &sphere_mesh.indices)
            .add_uniform("num_vertices", &(vertex_count as u32))
            .add_uniform("num_triangles", &(num_triangles as u32))
            .add_staging("new_vertices", &vec![Vec3::ZERO; vertex_count])
            .add_storage("noise_params_shape", &noise_params)
            .add_storage("noise_params_ridge", &noise_params)
            .add_storage("noise_params_ridge2", &noise_params)
            .add_uniform("max_strength", &0.)
            .add_uniform("num_craters", &0)
            .add_uniform("rim_steepness", &0.0)
            .add_uniform("rim_width", &0.0)
            .add_storage("craters", &[Crater::default(); MAX_CRATER])
            .add_staging(
                "normal_accumulators",
                &vec![NormalAccumulator::default(); vertex_count],
            )
            .add_pass::<AsteroidHeightComputeShader>(
                [num_workgroups as u32, 1, 1],
                &[
                    "vertices",
                    "new_vertices",
                    "num_vertices",
                    "noise_params_shape",
                    "noise_params_ridge",
                    "noise_params_ridge2",
                    "normal_accumulators",
                    "max_strength",
                    "num_craters",
                    "rim_steepness",
                    "rim_width",
                    "craters",
                ],
            )
            .add_pass::<NormalComputeShader>(
                [num_workgroups_normal as u32, 1, 1],
                &[
                    "new_vertices",
                    "indices",
                    "normal_accumulators",
                    "num_triangles",
                ],
            )
            .add_pass::<NormalizeNormalComputeShader>(
                [num_workgroups as u32, 1, 1],
                &[
                    "normal_accumulators",
                    "num_vertices",
                    "normals"
                ],
            )
            .one_shot()
            .build();

        world.insert_resource(sphere_mesh);
        worker
    }
}

fn send_perturb_strength_data(
    trigger: Trigger<PerturbStrengthChanged>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    sphere_mesh: Res<SphereMesh>
) {
    let ev = trigger.event();
    let perturb_strength = ev.0;
    
    let edge_length = (sphere_mesh.vertices[sphere_mesh.indices[0] as usize] - sphere_mesh.vertices[sphere_mesh.indices[1] as usize]).length();
    let max_perturb_strength = perturb_strength * edge_length / 2.;
    
    compute_worker.write_slice("max_strength", &[max_perturb_strength]);
    compute_worker.execute();
}

fn send_crater_settings_data(
    trigger: Trigger<CraterSettingsChanged>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    seed: ResMut<RngSeed>,
) {
    let ev = trigger.event();
    let crater_settings = &ev.0;
    let craters = crater_settings.get_craters(seed.0);

    compute_worker.write_slice("num_craters", &[craters.len() as u32]);
    compute_worker.write_slice("rim_steepness", &[crater_settings.get_rim_steepness()]);
    compute_worker.write_slice("rim_width", &[crater_settings.get_rim_width()]);
    compute_worker.write_slice("craters", &craters);
    compute_worker.execute();
}

fn send_simple_noise_settings_data(
    trigger: Trigger<SimpleNoiseSettingsChanged>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    seed: ResMut<RngSeed>,
) {
    let ev = trigger.event();
    let simple_noise_settings = &ev.0;
    
    let prng = PRNG::new(seed.0);
    let noise_params = simple_noise_settings.get_noise_params(prng);

    compute_worker.write_slice("noise_params_shape", &noise_params);
    compute_worker.execute();
}

fn send_ridge_noise_settings_data(
    trigger: Trigger<RidgeNoiseSettingsChanged>,
    mut compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    seed: ResMut<RngSeed>,
) {
    let ev = trigger.event();
    let ridge_noise_settings = &ev.0;
    let suffix = &ev.1;

    let prng = PRNG::new(seed.0);
    let noise_params = ridge_noise_settings.get_noise_params(prng);

    compute_worker.write_slice(&format!("noise_params_ridge{}",suffix), &noise_params);
    compute_worker.execute();
}


pub fn receive_data_after_compute(
    compute_worker: ResMut<AppComputeWorker<AsteroidComputeWorker>>,
    mut commands: Commands,
) {
    if compute_worker.ready() {
        let raw_vertices: Vec<[f32; 4]> = compute_worker.read_vec("new_vertices");
        let vertices: Vec<Vec3> = convert_array4_to_vec3(raw_vertices);

        let raw_normals: Vec<[f32; 4]> = compute_worker.read_vec("normals");
        let normals: Vec<Vec3> = convert_array4_to_vec3(raw_normals);

        commands.trigger(MeshDataAfterCompute(
            vertices,
            normals,
        ));
    }
}

fn convert_array4_to_vec3(raw: Vec<[f32; 4]>) -> Vec<Vec3> {
    raw.into_iter()
        .map(|[x, y, z, _]| Vec3::new(x, y, z))
        .collect()
}