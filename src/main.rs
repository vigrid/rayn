extern crate minifb;

use minifb::{ Window, WindowOptions, Key };

const WIDTH: usize = 960;
const HEIGHT: usize = 540;

const COLOR_MAGENTA: u32 = 0x00ff00ff;

fn clear(buffer: &mut Vec<u32>, color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

fn main() {
    let window_options = WindowOptions::default();
    let mut window = Window::new("RayN", WIDTH, HEIGHT, window_options).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear(&mut buffer, COLOR_MAGENTA);
        window.update_with_buffer(&buffer).unwrap();
    }
}

mod rayn {
    extern crate cgmath;

    use cgmath::{ Vector3 };

    #[allow(dead_code)]
    pub struct Ray {
        origin: Vector3<f32>,
        direction: Vector3<f32>,
    }
}