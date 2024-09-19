#import "shaders/utils.wgsl"::{NormalAccumulator,float_to_int}

@group(0) @binding(0) var<storage, read> new_vertices: array<vec3<f32>>;
@group(0) @binding(1) var<storage, read> indices: array<u32>;
@group(0) @binding(2) var<storage, read_write> normal_accumulators: array<NormalAccumulator>;
@group(0) @binding(3) var<storage, read_write> num_triangles: u32;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
     
       let triangle_index = global_id.x;
         if (triangle_index >= num_triangles) {
             return ;
         }

        let index_offset = triangle_index * 3u;
        let i0 = indices[index_offset];
        let i1 = indices[index_offset + 1u];
        let i2 = indices[index_offset + 2u];
    
        let v0 = new_vertices[i0];
        let v1 = new_vertices[i1];
        let v2 = new_vertices[i2];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        var  normal = normalize(cross(edge1, edge2));
         
        let normal_int = vec3<i32>(
          float_to_int(normal.x),
          float_to_int(normal.y),
          float_to_int(normal.z)
        );
      
        atomicAdd(&normal_accumulators[i0].x, normal_int.x);
        atomicAdd(&normal_accumulators[i0].y, normal_int.y);
        atomicAdd(&normal_accumulators[i0].z, normal_int.z);
        
        atomicAdd(&normal_accumulators[i1].x, normal_int.x);
        atomicAdd(&normal_accumulators[i1].y, normal_int.y);
        atomicAdd(&normal_accumulators[i1].z, normal_int.z);
        
        atomicAdd(&normal_accumulators[i2].x, normal_int.x);
        atomicAdd(&normal_accumulators[i2].y, normal_int.y);
        atomicAdd(&normal_accumulators[i2].z, normal_int.z);
}

