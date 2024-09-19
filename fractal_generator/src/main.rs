use fractal_generator::image::ppmimage::{PpmImage, RGB};
use fractal_generator::arithmetic::complex::Complex;

struct BoundOfComplexPlain{
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

fn mandelbrot_4_point(x: f64, y: f64, iterations: usize) -> usize{
    let mut z = Complex::default();
    let c = Complex::new(x,y);

    for i in 0..=iterations{
        if z.real > 2.0 || z.imaginary > 2.0 {//sqrt(z.r^2+z.i^2)>4.0
            return i;
        }
        z = z * z + c;
    }
    return iterations;
}


fn calculate_mandelbrot(iterations: usize, bounds: BoundOfComplexPlain, width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut rows = Vec::<Vec<usize>>::with_capacity(width);
    for y in 0..height{
        let mut row = Vec::<usize>::with_capacity(height);
        for x in 0..width{
            let point_x = bounds.x_min + (bounds.x_max - bounds.x_min) * (x as f64 /(width as f64));
            let point_y = bounds.y_min + (bounds.y_max - bounds.y_min) * (y as f64 /(height as f64));
            let escaped_at = mandelbrot_4_point(point_x, point_y, iterations);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}


fn render_mandelbrot(escape_vals: Vec<Vec<usize>>, filename : &str, width: usize, height: usize) {
    let mut image = PpmImage::new(width, height);
    for w in 0..width{        
        for h in 0..height{
            let color = match escape_vals[h][w] {
                0 ..= 2 => RGB::new(0, 6, 95),
                3 ..= 5 => RGB::new(50, 55, 126),
                6 ..= 10 => RGB::new(100, 104, 157),
                11 ..= 30 => RGB::new(250,250,250),
                31 ..= 100 => RGB::new(205, 172, 97),
                101 ..= 200 => RGB::new(206, 227, 220),
                201 ..= 400 => RGB::new(197, 236, 240),
                401 ..= 700 => RGB::new(171, 172, 156),
                _ => RGB::new(0, 0, 0),
            };
            image.encode_pixel(w, h, color);
        }
    }
    
    match image.write_file(filename){
        Err(err) => println!("{:?}", err),
        _ => ()
    }
  }

fn main(){
    let width = 1500;
    let height = 1500;
    let mandelbrot = calculate_mandelbrot(
    10000, 
    BoundOfComplexPlain { x_min: -1.5, x_max: 0.9, y_min: -1.1, y_max: 1.1 },    
    width,
    height, 
    );

    render_mandelbrot(mandelbrot, "image.ppm", width, height);
}

#[cfg(test)]
mod mandelbrot_tests{
    use super::*;
    use file_diff::diff_files;
    use std::fs::File;

    const WIDTH: usize = 1500;
    const HEIGHT: usize = 1500;

    #[test]
    fn is_generated_image_ok_1(){
        let mandelbrot = calculate_mandelbrot(
            10000, 
            BoundOfComplexPlain { x_min: -2.0, x_max:  1.0, y_min: -1.5, y_max: 1.2 },    
            WIDTH,
            HEIGHT, 
        );
        
        render_mandelbrot(mandelbrot, "testImages/mandelbrot1.ppm", WIDTH, HEIGHT);

        let mut generated_img = match File::open("testImages/mandelbrot1.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut expected_img = match File::open("testImages/expectedMandelbrot1.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        
        diff_files(&mut generated_img, &mut expected_img);
    }

    #[test]
    fn is_generated_image_ok_2(){
        let mandelbrot = calculate_mandelbrot(
            10000, 
            BoundOfComplexPlain { x_min: -1.0, x_max:  0.0, y_min: -0.5, y_max: 0.2 },
            WIDTH,
            HEIGHT, 
        );
        
        render_mandelbrot(mandelbrot, "testImages/mandelbrot2.ppm", WIDTH, HEIGHT);

        let mut generated_img = match File::open("testImages/mandelbrot2.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut expected_img = match File::open("testImages/expectedMandelbrot2.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        
        diff_files(&mut generated_img, &mut expected_img);
    }
    
    #[test]
    fn is_generated_image_ok_3(){
        let mandelbrot = calculate_mandelbrot(
            10000, 
            BoundOfComplexPlain { x_min: -0.78, x_max: -0.73, y_min: -0.05, y_max: 0.0 },
            WIDTH,
            HEIGHT, 
        );
        
        render_mandelbrot(mandelbrot, "testImages/mandelbrot3.ppm", WIDTH, HEIGHT);

        let mut generated_img = match File::open("testImages/mandelbrot3.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut expected_img = match File::open("testImages/expectedMandelbrot3.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        
        diff_files(&mut generated_img, &mut expected_img);
    }
}

// fn main() {
//     const FILENAME: &str = "image.ppm";
//     let width = 300;
//     let height  = 500;
//     let mut image = PpmImage::new(width, height);
    
//     for w in 0..width  {
//         for h in 0..height{
//             if h == 200{                
//                 let color = RGB::new(0 as u8, 0 as u8, 255 as u8);
//                 image.encode_pixel(w as usize, 2000 as usize, color);                
//             }
//             else {
//                 let color = RGB::new(((h+w)*50) as u8, 150 as u8, 255 as u8);
//                 image.encode_pixel(w as usize, h as usize, color);
//             }
//         }        
//     }

//     //image.print();

//     //println!("{:?}", image.pixels);
//     match image.write_file(FILENAME){
//         Err(err) => println!("{:?}", err),
//         _ => ()
//     }
//     //print!("{:?}", image.pixels_to_string());
// }

