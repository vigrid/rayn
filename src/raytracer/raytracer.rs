extern crate cgmath;

use cgmath::{InnerSpace, Vector3};

const TRACE_ITER_MAX: usize = 1024;
const NORMAL_EPSILON: f32 = 0.005;

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

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
}

pub enum Object {
    Plane(Vector3<f32>, f32),
    Sphere(Vector3<f32>, f32),
}

impl Ray {
    pub fn translate(&mut self, amount: f32) {
        self.origin += self.direction * amount;
    }
}

impl Camera {
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: Vector3 { x, y, z: 1.0 }.normalize(),
        }
    }
}

pub enum TraceResult {
    Hit(Ray, f32),
    Miss(f32),
    Fail,
}

pub fn trace<S>(sdf: S, ray: &mut Ray, min: f32, max: f32) -> TraceResult
where
    S: Fn(Vector3<f32>) -> f32,
{
    let mut iterations = TRACE_ITER_MAX;

    let mut total_distance = 0.0;

    while iterations > 0 {
        let distance = sdf(ray.origin);
        total_distance += distance;

        if distance < min {
            let normal = estimate_normal(&sdf, ray.origin, NORMAL_EPSILON);
            return TraceResult::Hit(
                Ray {
                    origin: ray.origin,
                    direction: normal,
                },
                1.0 - total_distance / max,
            );
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
pub fn estimate_normal<S>(scene: S, position: Vector3<f32>, epsilon: f32) -> Vector3<f32>
where
    S: Fn(Vector3<f32>) -> f32,
{
    let x1 = scene(Vector3 {
        x: position.x + epsilon,
        ..position
    });
    let x0 = scene(Vector3 {
        x: position.x - epsilon,
        ..position
    });
    let y1 = scene(Vector3 {
        y: position.y + epsilon,
        ..position
    });
    let y0 = scene(Vector3 {
        y: position.y - epsilon,
        ..position
    });
    let z1 = scene(Vector3 {
        z: position.z + epsilon,
        ..position
    });
    let z0 = scene(Vector3 {
        z: position.z - epsilon,
        ..position
    });

    let n = Vector3 {
        x: x1 - x0,
        y: y1 - y0,
        z: z1 - z0,
    };

    n.normalize()
}
