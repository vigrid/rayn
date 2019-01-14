extern crate minifb;
extern crate cgmath;
extern crate rayon;
extern crate num;

mod sdf;
mod raytracer;

use crate::raytracer::raytracer::*;
use crate::sdf::sdf::*;

use minifb::{ Window, WindowOptions, Key };
use rayon::prelude::*;

const NUM_SPHERES: usize = 4;
const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const SCALE: minifb::Scale = minifb::Scale::X4;

const COLOR_BLACK: u32 = 0x00000000;
const COLOR_MAGENTA: u32 = 0x00ff00ff;
const COLOR_WHITE: u32 = 0x00ffffff;

const TRACE_MIN: f32 = 0.01;
const TRACE_MAX: f32 = 30.0;

fn clear(buffer: &mut Vec<u32>, color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * k * 0.25
}

fn prepare_scene(scene_def: &mut Scene, time: f32) {
    let mut i = 0;
    for object in scene_def.objects.iter_mut() {
        match object {
            Object::Plane(pos, d) => {},
            Object::Sphere(pos, r) => {
                let o = i as f32 * 16.37;
                let new_pos = cgmath::Vector3 {
                    x: ((time + o) * 1.31).sin() * 2.0,
                    y: ((time + o) * 0.31).cos() * 4.0,
                    z: ((time + o) * 0.17).sin() * 2.0,
                };
                let new_r = 0.5;
                *object = Object::Sphere(new_pos, new_r);
                // println!("{}: {:?} {}", i, new_pos, new_r);
                i += 1;
            },
        }
    }
}

fn scene_sdf(scene_def: &Scene, position: cgmath::Vector3<f32>) -> f32 {
    let mut min_s: f32 = 1000.0;

    for object in scene_def.objects.iter() {
        match object {
            Object::Plane(pos, d) => min_s = smin(min_s, plane(position, *pos, *d), 3.5),
            Object::Sphere(pos, r) => min_s = smin(min_s, sphere(position - *pos, *r), 3.5),
        };
    };

    min_s
}

fn calculate_light(ray: Ray, t: f32) -> u32 {
    use cgmath::InnerSpace;

    let light_r = (cgmath::Vector3 { x: -0.25, y: -0.5, z: -1.0 }).normalize();
    let light_g = (cgmath::Vector3 { x: -0.55, y: -0.3, z: -1.0 }).normalize();
    let light_b = (cgmath::Vector3 { x:  0.25, y:  0.2, z:  0.2 }).normalize();

    let dot_product_r = cgmath::dot(ray.direction, light_r);
    let dot_product_g = cgmath::dot(ray.direction, light_g);
    let dot_product_b = cgmath::dot(ray.direction, light_b);

    let point = cgmath::Vector3 { x: 5.0, y: 2.0, z: -8.0 };

    let dot_camera = cgmath::dot(ray.direction, (ray.origin - point).normalize());

    let intensity_r = num::clamp(dot_product_r.powf(8.0) + 0.15, 0.0, 0.9);
    let intensity_g = num::clamp(dot_product_g.powf(6.0) + 0.15, 0.0, 0.9);
    let intensity_b = num::clamp(dot_product_b.powf(3.0) + 0.15, 0.0, 0.9);
    let intensity_camera = num::clamp(dot_camera.powf(80.0), 0.0, 1.0);

    let c = ((ray.origin.x * 2.0).floor() + (ray.origin.y * 2.0).floor() + (ray.origin.z * 2.0).floor()) as i32;

    let m = 8;

    let c = if c % 2 == 0 { 255.0 / (m as f32) } else { 192.0 / (m as f32) };

    let r = ((num::clamp(intensity_camera + intensity_r, 0.0, 1.0) * t * c * t) as u32) * m * 0x00010000;
    let g = ((num::clamp(intensity_camera + intensity_g, 0.0, 1.0) * t * c * t) as u32) * m * 0x00000100;
    let b = ((num::clamp(intensity_camera + intensity_b, 0.0, 1.0) * t * c * t) as u32) * m * 0x00000001;

    r + g + b
}

fn render(buffer: &mut Vec<u32>, time: f32) {
    extern crate cgmath;
    use cgmath::{ Vector3, InnerSpace };

    let fw = WIDTH as f32;
    let fh = HEIGHT as f32;
    let aspect_ratio = fw / fh;

    let mut scene_def = Scene {
        camera: Camera {
            origin: Vector3 { x: 0.0, y: 0.0, z: -5.0 },
            target: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            aspect_ratio,
            fov: 45.0
        },
        objects: vec![
            Object::Plane(Vector3 { x: 0.0, y: 1.0, z: 0.0 }, 3.0),
            Object::Plane(Vector3 { x: 0.0, y: -1.0, z: 0.0 }, 3.0),
        ],
    };

    for i in 0..NUM_SPHERES {
        scene_def.objects.push(Object::Sphere(Vector3 { x: 0.0, y: -1.0, z: 0.0 }, 2.0));
    }

    prepare_scene(&mut scene_def, time);

    let scene_fn = |pos| {
        scene_sdf(&scene_def, pos)
    };

    buffer.par_iter_mut().enumerate().for_each(|(n, pixel)| {
        let y = (n as usize) / WIDTH;
        let x = (n as usize) - (y * WIDTH);

        let fy = (y as f32) / fh * 2.0 - 1.0;
        let fx = ((x as f32) / fw * 2.0 - 1.0) * aspect_ratio;

        let mut ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: -5.0 },
            direction: Vector3 { x: fx, y: fy, z: 1.0 }.normalize(),
        };

        let color = match trace(scene_fn, &mut ray, TRACE_MIN, TRACE_MAX) {
            TraceResult::Hit(ray, t) => calculate_light(ray, t),
            TraceResult::Miss(_d) => COLOR_BLACK,
            TraceResult::Fail => COLOR_MAGENTA,
        };

        *pixel = color;
    });
}

fn main() {
    let mut window_options = WindowOptions::default();
    window_options.scale = SCALE;

    let mut window = Window::new("RayN", WIDTH, HEIGHT, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();

        render(&mut buffer, time);

        window.update_with_buffer(&buffer).unwrap();

        let elapsed = now.elapsed();
        let elapsed_ms = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;

        println!("{}", elapsed_ms as u64);

        time += (elapsed_ms as f32) / 1000.0;
    }
}