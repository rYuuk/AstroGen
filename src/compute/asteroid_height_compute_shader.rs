use bevy::prelude::TypePath;
use bevy::render::render_resource::ShaderRef;
use bevy_easy_compute::prelude::ComputeShader;

#[derive(TypePath)]
pub struct AsteroidHeightComputeShader;

impl ComputeShader for AsteroidHeightComputeShader {
    fn shader() -> ShaderRef {
        "shaders/compute_asteroid_height.wgsl".into()
    }
}

#[derive(TypePath)]
pub struct NormalComputeShader;

impl ComputeShader for NormalComputeShader {
    fn shader() -> ShaderRef {
        "shaders/compute_normals.wgsl".into()
    }
}