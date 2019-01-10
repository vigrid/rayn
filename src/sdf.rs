// Reference: https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
use cgmath::{ Vector3, InnerSpace };

#[allow(dead_code)]
pub fn sphere(p: Vector3<f32>, r: f32) -> f32 {
    p.magnitude() - r
}
