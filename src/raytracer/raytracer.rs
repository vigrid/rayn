extern crate cgmath;

use cgmath::{ Vector3, InnerSpace };

const TRACE_ITER_MAX: usize = 1024;
const NORMAL_EPSILON: f32 = 0.00005;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

pub struct Camera {
    pub origin: Vector3<f32>,
    pub target: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
}

impl Ray {
    pub fn translate(&mut self, amount: f32) {
        self.origin += self.direction * amount;
    }

    pub fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }
}

impl Camera {

}

#[test]
fn translate_works() {
    let mut ray = Ray {
        origin: Vector3 { x: 1.0, y: 2.0, z: 3.0 },
        direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
    };

    ray.translate(2.0);

    assert_eq!(ray.origin, Vector3 { x: 3.0, y: 2.0, z: 3.0 });
    assert_eq!(ray.direction, Vector3 { x: 1.0, y: 0.0, z: 0.0 });
}

#[test]
fn normalize_works() {
    let mut ray = Ray {
        origin: Vector3 { x: 1.0, y: 2.0, z: 3.0 },
        direction: Vector3 { x: 5.0, y: 0.0, z: 0.0 },
    };

    ray.normalize();

    assert_eq!(ray.origin, Vector3 { x: 1.0, y: 2.0, z: 3.0 });
    assert_eq!(ray.direction, Vector3 { x: 1.0, y: 0.0, z: 0.0 });
}


pub enum TraceResult {
    Hit(Ray, f32),
    Miss(f32),
    Fail,
}

pub fn trace(sdf: fn(Vector3<f32>, f32) -> f32, ray: &mut Ray, min: f32, max: f32, time: f32) -> TraceResult {
    let mut iterations = TRACE_ITER_MAX;

    let mut total_distance = 0.0;

    while iterations > 0 {
        let distance = sdf(ray.origin, time);
        total_distance += distance;

        if distance < min {
            let normal = estimate_normal(&sdf, ray.origin, NORMAL_EPSILON, time);
            return TraceResult::Hit(Ray { origin: ray.origin, direction: normal }, (iterations as f32) / (TRACE_ITER_MAX as f32));
        }
        if total_distance > max {
            return TraceResult::Miss(distance);
        }
        ray.translate(distance);

        iterations -= 1;
    }

    TraceResult::Fail
}

#[allow(dead_code)]
pub fn estimate_normal(scene: &fn(Vector3<f32>, f32) -> f32, position: Vector3<f32>, epsilon: f32, time: f32) -> Vector3<f32> {
    let x1 = scene(Vector3 { x: position.x + epsilon, ..position }, time);
    let x0 = scene(Vector3 { x: position.x - epsilon, ..position }, time);
    let y1 = scene(Vector3 { y: position.y + epsilon, ..position }, time);
    let y0 = scene(Vector3 { y: position.y - epsilon, ..position }, time);
    let z1 = scene(Vector3 { z: position.z + epsilon, ..position }, time);
    let z0 = scene(Vector3 { z: position.z - epsilon, ..position }, time);

    let n = Vector3 { x: x1 - x0, y: y1 - y0, z: z1 - z0 };

    n.normalize()
}