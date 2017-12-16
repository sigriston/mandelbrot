use std::mem;
use std::os::raw::c_void;

const SHADES_MULTIPLIER: i32 = 4;
const MAX_ITERATIONS: i32 = u8::max_value() as i32 * SHADES_MULTIPLIER;

struct RGBA(u8, u8, u8, u8);

struct Grayscale(u8);

impl From<Grayscale> for RGBA {
    fn from(mono: Grayscale) -> Self {
        match mono {
            Grayscale(gray) => RGBA(gray, gray, gray, u8::max_value()),
        }
    }
}

struct Canvas {
    width: u16,
    height: u16,
    buf: Vec<u8>,
}

impl Canvas {
    fn new(width: u16, height: u16) -> Self {
        Canvas {
            width,
            height,
            buf: vec![0u8; width as usize * height as usize * 4],
        }
    }

    fn set_pixel(&mut self, x: u16, y: u16, pixel: RGBA) {
        let pixel_offset = ((y as usize * self.width as usize) + x as usize) * 4;
        match pixel {
            RGBA(r, g, b, a) => {
                self.buf[pixel_offset] = r;
                self.buf[pixel_offset + 1] = g;
                self.buf[pixel_offset + 2] = b;
                self.buf[pixel_offset + 3] = a;
            },
        }
    }

    fn scale_x(&self, x: u16) -> f32 {
        x as f32 / self.width as f32 * 3.5 - 2.5
    }

    fn scale_y(&self, y: u16) -> f32 {
        y as f32 / self.height as f32 * 2.0 - 1.0
    }

    fn compute_mandelbrot(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.compute_mandelbrot_pixel(x, y);
                self.set_pixel(x, y, pixel);
            }
        }
    }

    fn compute_mandelbrot_pixel(&self, x: u16, y: u16) -> RGBA {
        let xs = self.scale_x(x);
        let ys = self.scale_y(y);
        let mut xf = 0.0f32;
        let mut yf = 0.0f32;
        let mut i = 0;

        while xf * xf + yf * yf < 4.0 && i < MAX_ITERATIONS {
            let xtemp = xf * xf - yf * yf + xs;
            yf = 2.0 * xf * yf + ys;
            xf = xtemp;
            i = i + 1;
        }

        RGBA::from(Grayscale(iter_to_shade(i)))
    }
}

fn iter_to_shade(i: i32) -> u8 {
    u8::max_value() - ((i / SHADES_MULTIPLIER) as u8)
}

#[no_mangle]
pub extern "C" fn mandelbrot(width: u16, height: u16) -> *mut c_void {
    let mut canvas = Canvas::new(width, height);
    canvas.compute_mandelbrot();
    let ptr = canvas.buf.as_mut_ptr();
    mem::forget(canvas);
    ptr as *mut c_void
}
