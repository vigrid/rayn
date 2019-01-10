extern crate minifb;
extern crate cgmath;

mod sdf;

use minifb::{ Window, WindowOptions, Key };

const WIDTH: usize = 1280 / 4;
const HEIGHT: usize = 720 / 4;
const SCALE: minifb::Scale = minifb::Scale::X4;

const COLOR_BLACK: u32 = 0x00000000;
const COLOR_MAGENTA: u32 = 0x00ff00ff;
const COLOR_WHITE: u32 = 0x00ffffff;

const TRACE_ITER_MAX: usize = 32;
const TRACE_MIN: f32 = 0.01;
const TRACE_MAX: f32 = 10.0;
const NORMAL_EPSILON: f32 = 0.00005;

fn clear(buffer: &mut Vec<u32>, color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

enum TraceResult {
    Hit(rayn::Ray),
    Miss,
    Exit,
}

fn min(a: f32, b: f32) -> f32 {
    match a < b {
        true => a,
        false => b
    }
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = max(k - (a - b).abs(), 0.0) / k;
    min(a, b) - h * h * k * 0.25
}

fn max(a: f32, b: f32) -> f32 {
    match a > b {
        true => a,
        false => b
    }
}

fn scene(position: cgmath::Vector3<f32>, time: f32) -> f32 {
    use cgmath::Vector3;

    let s1 = sdf::sphere(position + Vector3 { x: (time / 131.0).sin(), y: 0.0, z: 0.0 }, 0.7);
    let s2 = sdf::sphere(position - Vector3 { x: (time / 67.0).cos(), y: 0.65 * (time / 93.0).sin(), z: 0.0 }, 0.4);

    smin(s1, s2, 0.5)
}

fn trace(sdf: fn(cgmath::Vector3<f32>, f32) -> f32, ray: &mut rayn::Ray, min: f32, max: f32, time: f32) -> TraceResult {
    let mut iterations = TRACE_ITER_MAX;

    while iterations > 0 {
        let distance = sdf(ray.origin, time);
        if distance < min {
            let normal = sdf::estimate_normal(&sdf, ray.origin, NORMAL_EPSILON, time);
            return TraceResult::Hit(rayn::Ray { origin: ray.origin, direction: normal });
        }
        if distance > max {
            return TraceResult::Miss;
        }
        ray.translate(distance);

        iterations -= 1;
    }

    TraceResult::Exit
}

fn clamp(f: f32) -> f32 {
    if f < 0.0 {
        return 0.0;
    } else if f > 1.0 {
        return 1.0;
    }
    f
}

fn calculate_light(ray: rayn::Ray) -> u32 {
    use cgmath::InnerSpace;
    let light_r = (cgmath::Vector3 { x: -0.25, y: -0.5, z: -1.0 }).normalize();
    let light_g = (cgmath::Vector3 { x: -0.55, y: -0.3, z: -1.0 }).normalize();
    let light_b = (cgmath::Vector3 { x:  0.25, y:  0.2, z:  0.2 }).normalize();

    let dot_product_r = cgmath::dot(ray.direction, light_r);
    let dot_product_g = cgmath::dot(ray.direction, light_g);
    let dot_product_b = cgmath::dot(ray.direction, light_b);

    let intensity_r = clamp(dot_product_r.powf(8.0) + 0.05);
    let intensity_g = clamp(dot_product_g.powf(6.0) + 0.05);
    let intensity_b = clamp(dot_product_b.powf(3.0) + 0.05);

    let r = ((intensity_r * 255.0) as u32) * 0x00010000;
    let g = ((intensity_g * 255.0) as u32) * 0x00000100;
    let b = ((intensity_b * 255.0) as u32) * 0x00000001;

    r + g + b
}

fn render(buffer: &mut Vec<u32>, time: f32) {
    extern crate cgmath;
    use cgmath::{ Vector3 };

    let fw = WIDTH as f32;
    let fh = HEIGHT as f32;
    let aspect_ratio = fw / fh;

    let mut index: usize = 0;
    for y in 0..HEIGHT {
        let fy = (y as f32) / fh * 2.0 - 1.0;
        for x in 0..WIDTH {
            let fx = ((x as f32) / fw * 2.0 - 1.0) * aspect_ratio;

            let mut ray = rayn::Ray {
                origin: Vector3 { x: fx, y: fy, z: -5.0 },
                direction: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            };

            let color = match trace(scene, &mut ray, TRACE_MIN, TRACE_MAX, time) {
                TraceResult::Hit(ray) => calculate_light(ray),
                TraceResult::Miss => COLOR_BLACK,
                TraceResult::Exit => COLOR_BLACK,
            };

            buffer[index] = color;
            index += 1;
        }
    }
}

fn main() {
    let mut window_options = WindowOptions::default();
    window_options.scale = SCALE;

    let mut window = Window::new("RayN", WIDTH, HEIGHT, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();

        // clear(&mut buffer, COLOR_MAGENTA);
        render(&mut buffer, time);

        window.update_with_buffer(&buffer).unwrap();

        println!("{}", now.elapsed().subsec_millis());

        time += 1.0;
    }
}

mod rayn {
    extern crate cgmath;

    use cgmath::{ Vector3, InnerSpace };

    #[allow(dead_code)]
    pub struct Ray {
        pub origin: Vector3<f32>,
        pub direction: Vector3<f32>,
    }

    impl Ray {
        #[allow(dead_code)]
        pub fn translate(&mut self, amount: f32) {
            self.origin += self.direction * amount;
        }

        #[allow(dead_code)]
        pub fn normalize(&mut self) {
            self.direction = self.direction.normalize();
        }
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
}
