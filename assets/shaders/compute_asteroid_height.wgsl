#import "shaders/crater.wgsl"::calculateCraterDepth
#import "shaders/noise.wgsl"::{simpleNoise, smoothedRidgidNoise}

@group(0) @binding(0) var<storage, read> vertices: array<vec3<f32>>;
@group(0) @binding(1) var<storage, read_write> new_vertices: array<vec3<f32>>;
@group(0) @binding(2) var<uniform> num_vertices: u32;
@group(0) @binding(3) var<storage, read> noise_params_shape: array<vec4<f32>,3>;
@group(0) @binding(4) var<storage, read> noise_params_ridge: array<vec4<f32>,3>;
@group(0) @binding(5) var<storage, read> noise_params_ridge2: array<vec4<f32>,3>;
 
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        
        let index = global_id.x;
        if (index >= num_vertices) {
            return ;
        }

        let vertexPos = vertices[index];
        let elevationMultiplier = 0.01;
        
        let craterDepth = calculateCraterDepth(vertexPos);
        
        let shapeNoise = simpleNoise(vertexPos, noise_params_shape);
        
        // Ridge noise
        let ridgeNoise = smoothedRidgidNoise(vertexPos, noise_params_ridge);
        let ridge2 = smoothedRidgidNoise(vertexPos, noise_params_ridge2);
        
        let noiseSum = (shapeNoise + ridgeNoise + ridge2) * elevationMultiplier;
        let finalHeight = 1 + craterDepth + noiseSum;
        new_vertices[index] = vertexPos * finalHeight;
}