use bevy::math::Vec3;
use bevy::prelude::{App, Plugin, Resource, Update, World};
use bevy_easy_compute::prelude::{AppComputeWorker, AppComputeWorkerBuilder, AppComputeWorkerPlugin, ComputeWorker, ShaderType};
use crate::compute::asteroid_height_compute_shader::{AsteroidHeightComputeShader, NormalComputeShader, NormalizeNormalComputeShader};
use crate::compute::event_handler;
use crate::compute::event_handler::MeshDataAfterCompute;
use crate::settings::crater_settings::{Crater, MAX_CRATER};
use crate::sphere_mesh::SphereMesh;

pub struct AsteroidGeneratorPlugin;

impl Plugin for AsteroidGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AppComputeWorkerPlugin::<AsteroidComputeWorker>::default())
            .add_event::<MeshDataAfterCompute>()
            .add_systems(Update, (
                event_handler::on_crater_settings_changed,
                event_handler::on_ridge_settings_changed,
                event_handler::on_simple_noise_settings_changed,
                event_handler::receive_data_after_compute,
            ));
    }
}

#[repr(C)]
#[derive(ShaderType, Clone, Default, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NormalAccumulator {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
        let num_triangles = indices_len/3 ;

        let worker = AppComputeWorkerBuilder::new(self.world)
            .add_storage("vertices", &self.sphere_mesh.vertices)
            .add_staging("new_vertices", &vec![Vec3::ZERO; len])
            .add_uniform("num_vertices", &(len as u32))
            .add_storage("noise_params_shape", &noise_params)
            .add_storage("noise_params_ridge", &noise_params)
            .add_storage("noise_params_ridge2", &noise_params)
            .add_uniform("num_craters", &0)
            .add_uniform("rim_steepness", &0.0)
            .add_uniform("rim_width", &0.0)
            .add_storage("craters", &[Crater::default(); MAX_CRATER])
            .add_storage("indices", &self.sphere_mesh.indices)
            .add_uniform("num_indices", &(indices_len as u32))
            .add_staging("num_triangles", &(num_triangles as u32))
            .add_staging("normals", &vec![Vec3::ZERO; len])
            .add_staging("normal_accumulators", &vec![NormalAccumulator::default(); len])
          
            .add_pass::<AsteroidHeightComputeShader>([256, 1, 1], &[
                "vertices", "new_vertices", "num_vertices",
                "noise_params_shape", "noise_params_ridge", "noise_params_ridge2",
                "num_craters", "rim_steepness", "rim_width",
                "craters"
            ])
            .add_pass::<NormalComputeShader>([256, 1, 1], &[
                "new_vertices", "indices", "normal_accumulators", "num_triangles"
            ])
            .add_pass::<NormalizeNormalComputeShader>([256, 1, 1], &[
                "normal_accumulators", "num_vertices", "normals",
            ])
            .one_shot()
            .build();

        self.world.insert_resource(self.sphere_mesh);
        worker
    }
}
