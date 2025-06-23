extern crate raster;

use raster::{Color, Image};

fn main() {

    let mut img = Image::blank(800, 600);

    let c = (400, 300); 
    let r = 250; 

    for j in -r..= r { 
        for i in  -r..= r { 
            let x = i ; 
            let y = j ; 
            if x*x  + y*y  >= r*r - r && x*x + y*y <= r*r + r  {
                img.set_pixel(x + c.0, y + c.1, Color::rgb(255, 255, 255)).unwrap();
            }
        }
    }
    raster::save(&img, "circle.png").unwrap();
}


