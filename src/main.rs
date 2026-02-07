use minifb::{Key, ScaleMode, Window, WindowOptions};

/// Window initial size
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

/// Encodes color from u8 RGB to 32-bit pixel.
/// See documentation of window.update_with_buffer from Minifb for details.
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

struct Circle {
    x: i32,
    y: i32,
    radius: u32,
    // vx: i32,
    // vy: i32,
}

impl Circle {
    fn draw(&mut self, buffer: &mut [u32]) {}
}

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let azure_blue = from_u8_rgb(0, 127, 255);

    let mut window = Window::new(
        "Test - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = window.get_size();
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
        }

        for pixel in buffer.iter_mut() {
            *pixel = azure_blue;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
