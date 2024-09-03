#import "shaders/normal_utils.wgsl"::{NormalAccumulator,int_to_float}

@group(0) @binding(0) var<storage, read_write> normal_accumulators: array<NormalAccumulator>;
@group(0) @binding(1) var<uniform> num_vertices: u32;
@group(0) @binding(2) var<storage, read_write> normals: array<vec3<f32>>;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let total_threads = num_workgroups.x * 256u;
    let vertices_per_thread = (num_vertices + total_threads - 1u) / total_threads;
    
    for (var i = 0u; i < vertices_per_thread; i = i + 1u) {
        let vertex_index = global_id.x * vertices_per_thread + i;
        if (vertex_index >= num_vertices) {
            break;
        }

        var accumulated = vec3<f32>(
            int_to_float(atomicLoad(&normal_accumulators[vertex_index].x)),
            int_to_float(atomicLoad(&normal_accumulators[vertex_index].y)),
            int_to_float(atomicLoad(&normal_accumulators[vertex_index].z))
        );
        
        normals[vertex_index] = normalize(accumulated);
    }
}