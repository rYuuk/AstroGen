use bevy::prelude::{App, Plugin, Resource, Update, World};
use bevy_easy_compute::prelude::{AppComputeWorker, AppComputeWorkerPlugin, ComputeWorker};
use crate::compute::asteroid_height_compute_shader::AsteroidHeightComputeShader;
use crate::compute::event_handler;
use crate::compute::event_handler::HeightsAfterCompute;
use crate::sphere_mesh::SphereMesh;

pub struct AsteroidGeneratorPlugin;

impl Plugin for AsteroidGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AppComputeWorkerPlugin::<AsteroidComputeWorker>::default())
            .add_event::<HeightsAfterCompute>()
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

        let mut worker = AppComputeWorkerBuilder::new(self.world)
            .add_staging("vertices", &self.sphere_mesh.vertices)
            .add_staging("heights", &vec![0.0; len])
            .add_uniform("numVertices", &(len as u32))
            .add_uniform("testValue", &0.0)
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
            .add_pass::<AsteroidHeightComputeShader>([1024, 1, 1], &[
                "vertices", "heights", "numVertices", "testValue",
                "noise_params_shape", "noise_params_ridge", "noise_params_ridge2",
                "num_craters", "rim_steepness", "rim_width",
                "craters_centre", "craters_radius", "craters_floor_height", "craters_smoothness",
            ])
            .one_shot()
            .build();

        self.world.insert_resource(self.sphere_mesh);
        worker
    }
}
