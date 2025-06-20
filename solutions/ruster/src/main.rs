extern crate raster;

use raster::{Color, Image};

fn main() {
    let mut img = Image::blank(800, 600);
    let c = (400, 300); 
    let r = 300; 

    for j in -r..=r { 
        for i in -r..=r { 
            let x = i + c.0;
            let y = j + c.1;

            if x >= 0 && x < img.width as i32 && y >= 0 && y < img.height as i32 {
                if x * x + (y - c.1)*(y - c.1) <= r*r + r && x * x + (y - c.1)*(y - c.1) >= r*r - r {
                    img.set_pixel(x, y, Color::rgb(255, 255, 255)).unwrap();
                }
            }
        }
    }
    raster::save(&img, "circle.png").unwrap();
}
