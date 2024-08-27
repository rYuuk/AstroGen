#import "shaders/math.wgsl"::{smooth_max, smooth_min}

// Define the crater structure
struct Crater {
    centre: vec3<f32>,
    radius: f32,
    floor_height: f32,
    smoothness: f32,
};

@group(0) @binding(6)
var<uniform> num_craters: u32;

@group(0) @binding(7)
var<uniform> rim_steepness: f32;

@group(0) @binding(8)
var<uniform> rim_width: f32;

@group(0) @binding(9)
var<storage, read> craters_centre: array<vec3<f32>>;

@group(0) @binding(10)
var<storage, read> craters_radius: array<f32>;

@group(0) @binding(11)
var<storage, read> craters_floor_height: array<f32>;

@group(0) @binding(12)
var<storage, read> craters_smoothness: array<f32>;


fn calculateCraterDepth(vertexPos: vec3<f32>) -> f32 {
    var craterHeight: f32 = 0.0;

    for (var i: u32 = 0; i < num_craters; i = i + 1) {
        let x = length(vertexPos - craters_centre[i]) / craters_radius[i];
        
        let cavity = x * x - 1.0;
        let rimX = min(x - 1.0 - rim_width, 0.0);
        let rim = rim_steepness * rimX * rimX;
        
        var craterShape = smooth_max(cavity, craters_floor_height[i], craters_smoothness[i]);
        craterShape = smooth_min(craterShape, rim, craters_smoothness[i]);
        craterHeight += craterShape * craters_radius[i];
    }

    return craterHeight;
}