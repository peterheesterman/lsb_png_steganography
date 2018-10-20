
extern crate image;

use image::GenericImageView;
use image::{ImageBuffer};
/*
    Cases:
        - same size
        - input smaller
        - input larger
*/

pub fn hide(payload_path: &str, carrier_path: &str) {
    let payload = image::open(payload_path).unwrap();
    let _carrier = image::open(carrier_path).unwrap();
  
    println!("dimensions {:?}", payload.dimensions());
    println!("{:?}", payload.color());
    payload.save("./test-out-hide.png").unwrap();
}

pub fn reveal(_carrier_path: &str) {
    let img = ImageBuffer::from_fn(128, 128, |x, y| {
        if x % 2 == 0 || y % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    
    img.save("./test-out-reveal.png").unwrap();
}

#[cfg(test)]
mod tests;
