use bevy::math::Vec3;
use bevy::prelude::{App, Plugin, Resource, Update, World};
use bevy_easy_compute::prelude::{AppComputeWorker, AppComputeWorkerBuilder, AppComputeWorkerPlugin, ComputeWorker};
use crate::compute::asteroid_height_compute_shader::{AsteroidHeightComputeShader, NormalComputeShader};
use crate::compute::event_handler;
use crate::compute::event_handler::NewVerticesAfterCompute;
use crate::sphere_mesh::SphereMesh;

pub struct AsteroidGeneratorPlugin;

impl Plugin for AsteroidGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AppComputeWorkerPlugin::<AsteroidComputeWorker>::default())
            .add_event::<NewVerticesAfterCompute>()
            .add_systems(Update, (
                event_handler::on_crater_settings_changed,
                event_handler::on_ridge_settings_changed,
                event_handler::on_simple_noise_settings_changed,
                event_handler::receive_heights_after_compute,
            ));
    }
}

#[derive(Resource)]
pub struct AsteroidComputeWorker;

impl ComputeWorker for AsteroidComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        AsteroidComputeWorkerBuilder::new(world).build()
    }
}

struct AsteroidComputeWorkerBuilder<'a> {
    world: &'a mut World,
    sphere_mesh: SphereMesh,
}

impl<'a> AsteroidComputeWorkerBuilder<'a> {
    fn new(world: &'a mut World) -> Self {
        let sphere_mesh = SphereMesh::new(400);
        Self { world, sphere_mesh }
    }

    fn build(self) -> AppComputeWorker<AsteroidComputeWorker> {
        let len = self.sphere_mesh.vertices.len();
        let noise_params: Vec<[f32; 4]> = vec![[0.0; 4]; 3];

        let indices_len = self.sphere_mesh.indices.len();

        let worker = AppComputeWorkerBuilder::new(self.world)
            .add_storage("vertices", &self.sphere_mesh.vertices)
            .add_staging("new_vertices", &vec![0.0; len])
            .add_uniform("num_vertices", &(len as u32))
            .add_storage("noise_params_shape", &noise_params)
            .add_storage("noise_params_ridge", &noise_params)
            .add_storage("noise_params_ridge2", &noise_params)
            .add_uniform("num_craters", &0)
            .add_uniform("rim_steepness", &0.0)
            .add_uniform("rim_width", &0.0)
            .add_storage("craters_centre", &vec![Vec3::ZERO; 1000])
            .add_storage("craters_radius", &vec![0.0; 1000])
            .add_storage("craters_floor_height", &vec![0.0; 1000])
            .add_storage("craters_smoothness", &vec![0.0; 1000])
            // .add_storage("indices", &self.sphere_mesh.indices)
            // .add_uniform("num_indices", &(indices_len as u32))
            // .add_staging("normals", &vec![0.0; len])
            .add_pass::<AsteroidHeightComputeShader>([1024, 1, 1], &[
                "vertices", "new_vertices", "num_vertices",
                "noise_params_shape", "noise_params_ridge", "noise_params_ridge2",
                "num_craters", "rim_steepness", "rim_width",
                "craters_centre", "craters_radius", "craters_floor_height", "craters_smoothness",
            ])
            // .add_pass::<NormalComputeShader>([512, 1, 1], &[
            //     "new_vertices", "indices", "normals", "num_vertices", "num_indices"
            // ])
            .one_shot()
            .build();

        self.world.insert_resource(self.sphere_mesh);
        worker
    }
}
