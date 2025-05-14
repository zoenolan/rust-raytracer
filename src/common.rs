use rand::Rng;

// Constants

pub use std::f32::consts::PI;
pub use std::f32::INFINITY;

// Utility functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_float() -> f32 {
    // Return a random real in [0.0, 1.0)
    rand::thread_rng().gen()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    // Return a random real in [min, max)
    min + (max - min) * random_float()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
