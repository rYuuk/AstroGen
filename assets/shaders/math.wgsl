// Smooth minimum of two values, controlled by smoothing factor k
// When k = 0, this behaves identically to min(a, b)
fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
    let c = max(0.0, k);
    let h = max(0.0, min(1.0, (b - a + c) / (2.0 * c)));
    return a * h + b * (1.0 - h) - c * h * (1.0 - h);
}

// Smooth maximum of two values, controlled by smoothing factor k
// When k = 0, this behaves identically to max(a, b)
fn smooth_max(a: f32, b: f32, k: f32) -> f32 {
    let c = min(0.0, -k);  // Equivalent to C#'s `k = min(0, -k);`
    let h = max(0.0, min(1.0, (b - a + c) / (2.0 * c)));
    return a * h + b * (1.0 - h) - c * h * (1.0 - h);
}