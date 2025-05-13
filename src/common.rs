// Constants

pub use std::f32::consts::PI;
pub use std::f32::INFINITY;

// Utility functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
