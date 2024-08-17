@group(0) @binding(0)
 var<storage, read_write> vertices: array<vec3<f32>>;
 
 @group(0) @binding(1)
 var<storage, read_write> heights: array<f32>;
 
 @group(0) @binding(2)
 var<uniform> numVertices: u32;
 
 @group(0) @binding(3)
 var<storage, read_write> testValue: f32;
 
 @compute @workgroup_size(512)
 fn main(@builtin(global_invocation_id) id: vec3<u32>) {
     let index = id.x;
 
     if (index >= numVertices) {
         return;
     }
 
     let vertexPos = vertices[index];
     heights[index] = 1.0 + sin(vertexPos.y * testValue) * 0.05;
 }