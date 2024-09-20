use bevy::math::Vec3;
use bevy::prelude::{Event, TypePath};
use bevy::render::render_resource::{ShaderRef, ShaderType};
use bevy_easy_compute::prelude::ComputeShader;
use bytemuck::{Pod, Zeroable};
#[derive(Event)]
pub struct MeshDataAfterCompute(pub Vec<Vec3>, pub Vec<Vec3>);
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

#[derive(TypePath)]
pub struct NormalizeNormalComputeShader;

impl ComputeShader for NormalizeNormalComputeShader {
    fn shader() -> ShaderRef {
        "shaders/compute_normalize_normals.wgsl".into()
    }
}

#[repr(C)]
#[derive(ShaderType, Clone, Default, Copy, Pod, Zeroable)]
pub struct NormalAccumulator {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

