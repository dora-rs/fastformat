extern crate fastformat;

use fastformat::image::Image;

fn camera_read() -> ndarray::Array<u8, ndarray::Ix3> {
    // Dummy camera read

    let flat_image = (1..28).collect::<Vec<u8>>();
    println!(
        "Generate a camera image at address: {:?}",
        flat_image.as_ptr()
    );

    let image = Image::new_bgr8(flat_image, 3, 3, None);

    return image.to_nd_array().unwrap();
}

fn image_show(_frame: ndarray::ArrayView<u8, ndarray::Ix3>) {
    // Dummy image show

    println!("Showing an image.");
}

fn send_output(arrow_array: arrow::array::UnionArray) {
    // Dummy send output

    let image = Image::from_arrow(arrow_array).unwrap();

    println!(
        "Sending an image to dataflow. Image address is: {:?}",
        image.as_ptr()
    );
}

fn main() {
    // Read OpenCV Camera, default is nd_array BGR8
    let frame = camera_read();

    let image = Image::from_bgr8_nd_array(frame, Some("camera.left"));

    // Convert to RGB8, apply some filter (Black and White).
    let mut frame = image.to_rgb().to_nd_array().unwrap();

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

    let image = Image::from_rgb8_nd_array(frame, Some("camera.left.baw"));

    // Plot the image, you may only need a nd array view
    image_show(image.nd_array_view().unwrap());

    send_output(image.to_arrow().unwrap());
}
