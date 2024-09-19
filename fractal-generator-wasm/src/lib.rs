#![allow(dead_code)]
#![allow(unused_variables)]

mod utils;
mod complex;

use wasm_bindgen::prelude::*;
extern crate web_sys;
extern crate image;
use web_sys::ImageData;
use wasm_bindgen::Clamped;
use image::ImageBuffer;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct MandelBrot {
    width: u32,
    height: u32,
    transparency: u8,
    iterations: u32,
}

#[wasm_bindgen]
impl MandelBrot{
    pub fn new(width: u32, height: u32, iterations: u32, transparency: u8) -> Self{
        MandelBrot {
            width,
            height,
            transparency,
            iterations,
        }
    }

    pub fn set_transparency(&mut self, transparency: u8){
        self.transparency = transparency;
    }

    pub fn set_iterations(&mut self, iterations: u32){
        self.iterations = iterations;
    }

    fn mandelbrod_4_point(&self, x: f64, y: f64) -> u32{
        let mut z = complex::Complex::default();
        let c = complex::Complex::new(x,y);
        let iterations = self.iterations;

        for i in 0..=iterations{
            if z.real > 2.0 || z.imaginary > 2.0 {//sqrt(z.r^2+z.i^2)>4.0
                return i;
            }
            z = z * z + c;
        }
        return iterations;
    }

    pub fn to_img(&self, zoom: u32, zoom_point_x: f64, zoom_point_y: f64) -> web_sys::ImageData {
        let factor = self.width as f64 / self.height as f64;
        let half_width = self.width as f64 / 2.0;
        let half_height = self.height as f64 / 2.0;
        let deletion = f64::powf(0.93, zoom as f64);

        let mut image = ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let position_x = zoom_point_x + ((x as f64 - self.width as f64 / 2.0) / half_width) * 2.0 * factor * deletion;
            let position_y = zoom_point_y + ((y as f64 - self.height as f64 / 2.0) / half_height) * 2.0 * deletion;
            //log!("x {}, y {}", position_x, position_y);
            let escaped_at = self.mandelbrod_4_point(position_x, position_y);

            if escaped_at == 0 {
                *pixel = image::Rgba([255, 255, 255, self.transparency]);
            } else {
                let (r,g,b) = Self::set_color(escaped_at as f32 / 100.0);
                *pixel = image::Rgba([r, g, b, self.transparency]);
            }
        }

        let mut image = Img::new(self.width, self.height, image.to_vec());
        image.get_img_buffer()
    }

    fn set_color(escape_point: f32) -> (u8, u8, u8){
        let c = 1.0 - (2.0 * escape_point - 1.0).abs();
        let m = escape_point - c/2.0;

        let r = (m * 255.0) as u8;
        let g = (m * 255.0) as u8;
        let b = ((c + m) * 255.0) as u8;
        (r, g, b)
    }
}

#[wasm_bindgen]
pub struct Img{
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl Img{
    pub fn new(width: u32, height: u32, buffer: Vec<u8>) -> Img{
        Img {
            width,
            height,
            buffer,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_img_buffer(&mut self) -> ImageData{
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut self.buffer), self.width, self.height).unwrap()
    }
}
