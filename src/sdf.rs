// Reference: https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
use cgmath::{ Vector3, InnerSpace };

#[allow(dead_code)]
pub fn sphere(p: Vector3<f32>, r: f32) -> f32 {
    p.magnitude() - r
}

#[allow(dead_code)]
pub fn estimate_normal(scene: &fn(Vector3<f32>) -> f32, position: Vector3<f32>, epsilon: f32) -> Vector3<f32> {
    let x1 = scene(Vector3 { x: position.x + epsilon, ..position });
    let x0 = scene(Vector3 { x: position.x - epsilon, ..position });
    let y1 = scene(Vector3 { y: position.y + epsilon, ..position });
    let y0 = scene(Vector3 { y: position.y - epsilon, ..position });
    let z1 = scene(Vector3 { z: position.z + epsilon, ..position });
    let z0 = scene(Vector3 { z: position.z - epsilon, ..position });

    let n = Vector3 { x: x1 - x0, y: y1 - y0, z: z1 - z0 };

    n.normalize()
}