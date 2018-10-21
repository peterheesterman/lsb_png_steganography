
extern crate image;

use image::{ImageBuffer, GenericImageView};

#[cfg(test)]
mod tests;

mod hider;

// Same size and type
pub fn hide(payload_path: &str, carrier_path: &str) {

    let payload = image::open(payload_path).unwrap();
    let carrier = image::open(carrier_path).unwrap();

    let mut img: image::RgbImage = ImageBuffer::new(128, 128);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
      let payload_pixel = payload.get_pixel(x, y);
      let carrier_pixel = carrier.get_pixel(x, y);

      *pixel = image::Rgb([
        hider::capture_most_significant_bits(carrier_pixel.data[0], payload_pixel.data[0]),
        hider::capture_most_significant_bits(carrier_pixel.data[1], payload_pixel.data[1]),
        hider::capture_most_significant_bits(carrier_pixel.data[2], payload_pixel.data[2])
      ]);
    }

    img.save("./test-out-hide-made.png").unwrap();
}

pub fn reveal(_carrier_path: &str) {
    // let img = ImageBuffer::from_fn(128, 128, |x, y| {
    //     if x % 2 == 0 || y % 2 == 0 {
    //         image::Rgba([111u8, 111u8, 111u8, 255u8])
    //     } else {
    //         image::Rgba([111u8, 111u8, 111u8, 255u8])
    //     }
    // });
    
    // img.save("./test-out-reveal.png").unwrap();
}
