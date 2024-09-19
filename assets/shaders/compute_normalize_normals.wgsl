#import "shaders/utils.wgsl"::{NormalAccumulator,int_to_float}

@group(0) @binding(0) var<storage, read_write> normal_accumulators: array<NormalAccumulator>;
@group(0) @binding(1) var<uniform> num_vertices: u32;
@group(0) @binding(2) var<storage, read_write> normals: array<vec3<f32>>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
     let vertex_index = global_id.x;
            if (vertex_index >= num_vertices) {
                return ;
            }

        var accumulated = vec3<f32>(
            int_to_float(atomicLoad(&normal_accumulators[vertex_index].x)),
            int_to_float(atomicLoad(&normal_accumulators[vertex_index].y)),
            int_to_float(atomicLoad(&normal_accumulators[vertex_index].z))
        );
        
        normals[vertex_index] = normalize(accumulated);
}