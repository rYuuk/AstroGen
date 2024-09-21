use bevy::prelude::TypePath;
use bevy::render::render_resource::ShaderRef;
use bevy_easy_compute::prelude::ComputeShader;

#[derive(TypePath)]
pub struct AsteroidShapeComputeShader;

impl ComputeShader for AsteroidShapeComputeShader {
    fn shader() -> ShaderRef {
        "shaders/compute_asteroid_shape.wgsl".into()
    }
}

#[derive(TypePath)]
pub struct NormalComputeShader;

impl ComputeShader for NormalComputeShader {
    fn shader() -> ShaderRef {
        "shaders/compute_normals.wgsl".into()
    }
}

#[derive(TypePath)]
pub struct NormalizeNormalComputeShader;

impl ComputeShader for NormalizeNormalComputeShader {
    fn shader() -> ShaderRef {
        "shaders/compute_normalize_normals.wgsl".into()
    }
}



