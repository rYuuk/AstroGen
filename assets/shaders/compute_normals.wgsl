@group(0) @binding(0) 
var<storage, read> new_vertices: array<vec3<f32>>;

@group(0) @binding(1) 
var<storage, read> indices: array<u32>;

@group(0) @binding(2) 
var<storage, read_write> normals: array<vec3<f32>>;

@group(0) @binding(3) 
var<uniform> num_vertices: u32;

@group(0) @binding(4) 
var<uniform> num_indices: u32;

@compute @workgroup_size(512)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= num_vertices) {
        return;
    }

    var accumulated_normal = vec3<f32>(0.0, 0.0, 0.0);

    // Iterate through all triangles
    for (var i: u32 = 0u; i < num_indices; i += 3u) {
        let i0 = indices[i];
        let i1 = indices[i + 1u];
        let i2 = indices[i + 2u];

        // Check if the current vertex is part of this triangle
        if (index == i0 || index == i1 || index == i2) {
            let v0 = new_vertices[i0];
            let v1 = new_vertices[i1];
            let v2 = new_vertices[i2];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = cross(edge1, edge2);

            accumulated_normal += normal;
        }
    }

    // Normalize the accumulated normal
    normals[index] = normalize(accumulated_normal);
}