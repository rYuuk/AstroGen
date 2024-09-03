const FLOAT_SCALE: f32 = 1000.0;

struct NormalAccumulator {
    x: atomic<i32>,
    y: atomic<i32>,
    z: atomic<i32>,
}

fn int_to_float(v: i32) -> f32 {
    return f32(v) / FLOAT_SCALE;
}

fn float_to_int(v: f32) -> i32 {
    return i32(v * FLOAT_SCALE);
}

