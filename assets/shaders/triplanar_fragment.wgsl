#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

@group(2) @binding(0) var<uniform> scale: f32;
@group(2) @binding(1) var<uniform> blend_sharpness: f32;
@group(2) @binding(2) var main_texture: texture_2d<f32>;
@group(2) @binding(3) var main_sampler: sampler;
@group(2) @binding(4) var normal_map: texture_2d<f32>;
@group(2) @binding(5) var normal_sampler: sampler;
@group(2) @binding(6) var<uniform> light_direction: vec3f;

fn unpack_normal(packed: vec4<f32>) -> vec3<f32> {
    let unpacked = packed.xyz * 2.0 - 1.0;
    return normalize(unpacked);
}

fn triplanar_normal(position: vec3<f32>, surface_normal: vec3<f32>) -> vec3<f32> {
    let tnormal_x = unpack_normal(textureSample(normal_map, normal_sampler, position.zy * scale));
    let tnormal_y = unpack_normal(textureSample(normal_map, normal_sampler, position.xz * scale));
    let tnormal_z = unpack_normal(textureSample(normal_map, normal_sampler, position.xy * scale));

    let tnormal_x_adj = vec3<f32>(tnormal_x.xy + surface_normal.zy, tnormal_x.z * surface_normal.x);
    let tnormal_y_adj = vec3<f32>(tnormal_y.xy + surface_normal.xz, tnormal_y.z * surface_normal.y);
    let tnormal_z_adj = vec3<f32>(tnormal_z.xy + surface_normal.xy, tnormal_z.z * surface_normal.z);

    let weight = pow(abs(surface_normal), vec3<f32>(blend_sharpness));
    let weight_normalized = weight / dot(weight, vec3<f32>(1.0));

    return normalize(
        tnormal_x_adj.zyx * weight_normalized.x +
            tnormal_y_adj.xzy * weight_normalized.y +
            tnormal_z_adj.xyz * weight_normalized.z
    );
}

@fragment
fn fragment(@location(0) worldPos : vec3f,
            @location(1) normal : vec3f,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let uv_x = worldPos.zy * scale;
    let uv_y = worldPos.xz * scale;
    let uv_z = worldPos.xy * scale;
    
    let col_x = textureSample(main_texture, main_sampler, uv_x);
    let col_y = textureSample(main_texture, main_sampler, uv_y);
    let col_z = textureSample(main_texture, main_sampler, uv_z);
    
    let blend_weight = pow(abs(normal), vec3<f32>(blend_sharpness));
    let blend_weight_normalized = blend_weight / dot(blend_weight, vec3<f32>(1.0));
    
    let col = col_x * blend_weight_normalized.x +
    col_y * blend_weight_normalized.y +
    col_z * blend_weight_normalized.z;
    
    let lighting_normal = triplanar_normal(worldPos, normal);
    let light_shading = max(dot(lighting_normal, light_direction), 0.0);
    
    return vec4<f32>(1.0, 1.0, 1.0, 1.0) * light_shading;
}