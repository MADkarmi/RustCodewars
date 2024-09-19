use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::str;

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB{
    pub fn new(r: u8, g: u8, b: u8) -> RGB{
        if(r <= u8::MAX) && (g <= u8::MAX) && (g<=u8::MAX) {
            RGB { r, g, b }
        }
        else {
            RGB { r:0, g:0, b:0 }
        }       
    }
}

pub struct PpmImage {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl PpmImage{
    pub fn new(width:usize, height:usize) -> PpmImage{
        let buffer = vec![0; width*height];
        PpmImage {width, height, pixels: buffer}
    }

    pub fn encode_pixel(&mut self, w: usize, h: usize, color: RGB){
        if w < self.width && h < self.height {            
            self.pixels[h * self.width + w ] = (color.r as u32) << 16 |
                                              (color.g as u32) << 8  |
                                              (color.b as u32) << 0;            
        }
    }

    pub fn decode_pixel(&self, w: usize, h: usize) -> RGB{
        if w < self.width && h < self.height {
            let offset = h * self.width + w;            
            let r = (self.pixels[offset] >> 16) as u8;
            let g = (self.pixels[offset] >> 8) as u8;
            let b = (self.pixels[offset] >> 0) as u8;
            RGB{r: r, g: g, b: b}
        } else {
            RGB { r:0, g:0, b:0 }
        }
    }    

    pub fn print(&self){
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        println!("{}", header);
        for w in 0..self.width{
            for h in 0..self.height{                           
                let rgb = self.decode_pixel(w, h);
                print!("({:?} {:?} {:?}) ", rgb.r, rgb.g, rgb.b);                
            }
            print!("\n");
        }
    }

    fn decode_pixel_to_string(&self, pixel: &u32) -> String{        
        let r = (((pixel >> 16) as u8) as u32).to_string();
        let g = (((pixel >> 8) as u8) as u32).to_string();
        let b = (((pixel >> 0) as u8) as u32).to_string();
        format!("({}, {}, {})", r,g,b)
    }

    pub fn pixels_to_string(&self) -> String{
        let mut string = String::new();
        for pixel in &self.pixels{
            string+=self.decode_pixel_to_string(pixel).as_str();
        }
        string        
    }

    fn decode_pixel_for_buffer(&self, pixel: &u32) -> Vec<u8>{        
        vec![(pixel >> 16) as u8, (pixel >> 8) as u8, (pixel >> 0) as u8]
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()>{
        let path = Path::new(filename);
        let mut file = File::create(&path)?; 
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        
        for i in &self.pixels{            
            let buffer = &self.decode_pixel_for_buffer(i);
            file.write(&buffer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod image_methods_tests{
    use super::*;
    use file_diff::diff_files;
    use std::fs::File;

    const WIDTH: usize = 255;
    const HEIGHT: usize = 255;

    #[test] //test1
    fn is_saved_image_correct(){
        let mut image = PpmImage::new(WIDTH, HEIGHT);
        
        for w in 0..WIDTH  {
            for h in 0..HEIGHT{
                if h==10{
                    let color = RGB::new(250 as u8, 20 as u8, ((h+w)*5) as u8);
                    image.encode_pixel(w as usize, h as usize, color);                
                }            
            }        
        }

        match image.write_file("testImages/testIsSavedImageCorrect.ppm"){
            Err(err) => println!("{:?}", err),
            _ => ()
        }

        let mut generated_img = match File::open("testImages/testIsSavedImageCorrect.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut expected_img = match File::open("testImages/expectedImg.ppm") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        
        diff_files(&mut generated_img, &mut expected_img);
    }

    #[test] //test2
    fn is_generated_encoding_correct(){
        let width = 5;
        let height  = 4;
        let mut image = PpmImage::new(width, height);       
        
        for w in 0..width  {
            for h in 0..height{
                if h==w{
                    let color = RGB::new(250 as u8, 250 as u8, ((h+w)*5) as u8);
                    image.encode_pixel(w as usize, h as usize, color);                
                }            
            }        
        }

        let generated = image.pixels_to_string();

        let expected = "(250, 250, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(250, 250, 10)(0, 0, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(250, 250, 20)(0, 0, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(0, 0, 0)(250, 250, 30)(0, 0, 0)";

        assert_eq!(expected, generated.as_str());        
    }
}