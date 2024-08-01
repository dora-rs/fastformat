extern crate fastformat;

use fastformat::image::Image;

fn main() {
    let pixels = (0..27).collect::<Vec<u8>>();
    println!("{:?}", pixels.as_ptr());

    let rgb8_image = Image::new_rgb8(pixels, 3, 3, None);
    println!("{:?}", rgb8_image.as_ptr());

    let bgr8_image = rgb8_image.to_bgr();
    println!("{:?}", bgr8_image.as_ptr());

    let rgb8_image = bgr8_image.to_rgb();
    println!("{:?}", rgb8_image.as_ptr());

    let rgb8_nd_array = rgb8_image.to_nd_array();
    println!("{:?}", rgb8_nd_array.as_ptr());

    let bgr8_image = Image::from_rgb8_nd_array(rgb8_nd_array, None).to_bgr();
    println!("{:?}", bgr8_image.as_ptr());

    let arrow_array = bgr8_image.to_arrow();
    let bgr8_image = Image::from_arrow(arrow_array).to_bgr();
    println!("{:?}", bgr8_image.as_ptr());
}
