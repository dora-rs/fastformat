extern crate fastformat;

use fastformat::image::Image;

fn camera_read() -> ndarray::Array<u8, ndarray::Ix3> {
    // Dummy camera read

    let flat_image = (110..137).collect::<Vec<u8>>();
    println!(
        "Generate a camera image at address: {:?}",
        flat_image.as_ptr()
    );

    let image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();

    return image.bgr8_into_ndarray().unwrap();
}

fn image_show(frame: ndarray::ArrayView<u8, ndarray::Ix3>) {
    // Dummy image show

    println!("{:?}", frame);
}

fn send_output(arrow_array: arrow::array::ArrayData) {
    // Dummy send output

    let image = Image::from_arrow(arrow_array).unwrap();

    println!(
        "Sending an image to dataflow. Image address is: {:?}",
        image.data.as_ptr()
    );
}

fn main() {
    // Read OpenCV Camera, default is ndarray BGR8
    let frame = camera_read();

    let image = Image::bgr8_from_ndarray(frame, Some("camera.left")).unwrap();

    // Convert to RGB8, apply some filter (Black and White).
    let mut frame = image.into_rgb8().unwrap().rgb8_into_ndarray().unwrap();

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

    let image = Image::rgb8_from_ndarray(frame, Some("camera.left.baw")).unwrap();

    // Plot the image, you may only need a ndarray_view
    image_show(image.rgb8_into_ndarray_view().unwrap());

    send_output(image.into_arrow().unwrap());
}
