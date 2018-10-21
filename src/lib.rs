
extern crate image;

use image::{ ImageBuffer, GenericImageView };

#[cfg(test)]
mod tests;

mod hider;
mod revealer;

// Same size and type
// Copy all the payload or the limit if the carrier size.
pub fn hide(payload_path: &str, carrier_path: &str) 
    -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {

    let payload = image::open(payload_path).unwrap();
    let carrier = image::open(carrier_path).unwrap();

    let (carrier_x_limit, carrier_y_limit) = carrier.dimensions();

    let (x_limit, y_limit) = hider::get_limits_for_image(
        payload.dimensions(), 
        (carrier_x_limit, carrier_y_limit)
    );

    let mut img: image::RgbImage = ImageBuffer::new(carrier_x_limit, carrier_y_limit);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if x < x_limit && y < y_limit {
            let payload_pixel = payload.get_pixel(x, y);
            let carrier_pixel = carrier.get_pixel(x, y);

            *pixel = image::Rgb([
                hider::capture_most_significant_bits(carrier_pixel.data[0], payload_pixel.data[0]),
                hider::capture_most_significant_bits(carrier_pixel.data[1], payload_pixel.data[1]),
                hider::capture_most_significant_bits(carrier_pixel.data[2], payload_pixel.data[2])
            ]);
        } else if x < carrier_x_limit && y < carrier_y_limit {
            let carrier_pixel = carrier.get_pixel(x, y);

            *pixel = image::Rgb([
                carrier_pixel.data[0],
                carrier_pixel.data[1],
                carrier_pixel.data[2]
            ]);
        }
    }

    img
}

pub fn reveal(carrier_path: &str) 
    -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {

    let carrier = image::open(carrier_path).unwrap();

    let (x_limit, y_limit) = carrier.dimensions();

    let mut img: image::RgbImage = ImageBuffer::new(x_limit, y_limit);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let carrier_pixel = carrier.get_pixel(x, y);

        *pixel = image::Rgb([
            revealer::promote_least_significant_bits(carrier_pixel.data[0]),
            revealer::promote_least_significant_bits(carrier_pixel.data[1]),
            revealer::promote_least_significant_bits(carrier_pixel.data[2])
        ]);
    }

    img
}
