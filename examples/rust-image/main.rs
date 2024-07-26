extern crate fastformat;

use arrow::array::UnionArray;

use fastformat::image::{
    Encoding::{BGR8, RGB8},
    Image,
};

fn camera_read() -> ndarray::Array<u8, ndarray::Ix3> {
    let flat_image = (1..28).collect::<Vec<u8>>();
    println!(
        "Generate a camera image at address: {:?}",
        flat_image.as_ptr()
    );

    let image = Image::from_flat(flat_image, 3, 3, BGR8);

    return image.to_nd_array(BGR8).unwrap();
}

fn image_show(_frame: &ndarray::Array<u8, ndarray::Ix3>) {}

fn send_output(arrow_array: UnionArray) {
    let image = Image::from_arrow_array(arrow_array);

    println!(
        "Sending an image to dataflow. Image address is: {:?}",
        image.as_ptr()
    );
}

fn main() {
    // Read OpenCV Camera, default is nd_array BGR8
    let frame = camera_read();

    let image = Image::from_nd_array(frame, BGR8);

    // Convert to RGB8 to plot it with OpenCV imshow
    let mut frame = image.to_nd_array(RGB8).unwrap();

    // Apply some filter
    for i in 0..frame.shape()[0] {
        for j in 0..frame.shape()[1] {
            let mean =
                (frame[[i, j, 0]] as f32 + frame[[i, j, 1]] as f32 + frame[[i, j, 2]] as f32) / 3.0;

            if mean > 128.0 {
                frame[[i, j, 0]] = 255;
                frame[[i, j, 1]] = 255;
                frame[[i, j, 2]] = 255;
            } else {
                frame[[i, j, 0]] = 0;
                frame[[i, j, 1]] = 0;
                frame[[i, j, 2]] = 0;
            }
        }
    }

    // Plot the image
    image_show(&frame);

    // Convert it back to arrow to send it to dataflow
    let image = Image::from_nd_array(frame, BGR8);
    let arrow_array = image.to_arrow_array().unwrap();

    send_output(arrow_array);
}
