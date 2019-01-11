// Reference: https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
use cgmath::{ Vector3, InnerSpace };

pub fn sphere(p: Vector3<f32>, r: f32) -> f32 {
    p.magnitude() - r
}

pub fn unit_box(p: Vector3<f32>) -> f32 {
    let v = Vector3::new(
        (p.x.abs() - 1.0).max(0.0),
        (p.y.abs() - 1.0).max(0.0),
        (p.z.abs() - 1.0).max(0.0),
    );

    v.magnitude()
}

pub fn repeat(p: Vector3<f32>, r: Vector3<f32>) -> Vector3<f32> {
    Vector3 {
        x: fmod(p.x, r.x) - 0.5 * r.x,
        y: fmod(p.y, r.y) - 0.5 * r.y,
        z: fmod(p.z, r.z) - 0.5 * r.z,
    }
}

pub fn fmod(x: f32, y: f32) -> f32 {
    x - y * ((x / y).floor())
}
