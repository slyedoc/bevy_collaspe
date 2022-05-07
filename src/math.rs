pub fn range_lerp(value: f32, min1: f32, max1: f32, min2: f32, max2: f32) -> f32 {
    let value_norm = inverse_lerp(min1, max1, value);
    lerp( min2, max2, value_norm)
}

pub fn lerp(min: f32, max: f32, parameter: f32) -> f32 {
    min + (max - min) * parameter
}

pub fn inverse_lerp( min: f32, max: f32, value: f32) -> f32 {
    (value-min) / (max-min)
}

