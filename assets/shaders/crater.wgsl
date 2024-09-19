#import "shaders/utils.wgsl"::{smooth_max, smooth_min}

struct Crater {
    centre: array<f32,3>,
    radius: f32,
    floor_height: f32,
    smoothness: f32,
};

@group(0) @binding(6) var<uniform> num_craters: u32;
@group(0) @binding(7) var<uniform> rim_steepness: f32;
@group(0) @binding(8) var<uniform> rim_width: f32;
@group(0) @binding(9) var<storage, read> craters: array<Crater>;

fn calculateCraterDepth(vertexPos: vec3<f32>) -> f32 {
    var craterHeight: f32 = 0.0;

    for (var i: u32 = 0; i < num_craters; i = i + 1) {
        let centre = vec3(craters[i].centre[0],craters[i].centre[1],craters[i].centre[2]);
        let x = length(vertexPos - centre) / craters[i].radius;

        let cavity = x * x - 1.0;
        let rimX = min(x - 1.0 - rim_width, 0.0);
        let rim = rim_steepness * rimX * rimX;

        var craterShape = smooth_max(cavity, craters[i].floor_height, craters[i].smoothness);
        craterShape = smooth_min(craterShape, rim, craters[i].smoothness);
        craterHeight += craterShape * craters[i].radius;
    }

    return craterHeight;
}