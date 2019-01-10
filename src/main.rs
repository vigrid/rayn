extern crate minifb;

mod sdf;

use minifb::{ Window, WindowOptions, Key };

const WIDTH: usize = 960;
const HEIGHT: usize = 540;

const COLOR_BLACK: u32 = 0x00000000;
const COLOR_MAGENTA: u32 = 0x00ff00ff;
const COLOR_WHITE: u32 = 0x00ffffff;

const TRACE_MIN: f32 = 0.001;
const TRACE_MAX: f32 = 100.0;

fn clear(buffer: &mut Vec<u32>, color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

enum TraceResult {
    Hit,
    Miss,
    Exit,
}

fn trace(ray: &mut rayn::Ray, min: f32, max: f32) -> TraceResult {
    let mut iterations = 128;

    while iterations > 0 {
        let distance = sdf::sphere(ray.origin, 1.0);
        if distance < min {
            return TraceResult::Hit;
        }
        if distance > max {
            return TraceResult::Miss;
        }
        ray.translate(distance);

        iterations -= 1;
    }

    TraceResult::Exit
}

fn render(buffer: &mut Vec<u32>) {
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

            let color = match trace(&mut ray, TRACE_MIN, TRACE_MAX) {
                TraceResult::Hit => COLOR_WHITE,
                TraceResult::Miss => COLOR_BLACK,
                TraceResult::Exit => COLOR_MAGENTA,
            };

            buffer[index] = color;
            index += 1;
        }
    }
}

fn main() {
    let window_options = WindowOptions::default();
    let mut window = Window::new("RayN", WIDTH, HEIGHT, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();

        clear(&mut buffer, COLOR_MAGENTA);
        render(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();

        println!("{}", now.elapsed().subsec_millis());
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
