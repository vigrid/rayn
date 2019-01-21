// Reference: https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
use cgmath::{InnerSpace, Vector3};

pub fn sphere(p: Vector3<f32>, r: f32) -> f32 {
    p.magnitude() - r
}

pub fn plane(p: Vector3<f32>, n: Vector3<f32>, d: f32) -> f32 {
    cgmath::dot(p, n) + d
}
