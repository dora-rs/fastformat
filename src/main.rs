pub mod image;

use arrow::array::{UInt8Array, UnionArray};

use crate::image::{
    Image,
    Encoding::{
        RGB8,
        BGR8,
    },
};

fn camera_read() -> ndarray::Array<u8, ndarray::Ix3> {
    let flat_image = (1..28).collect::<Vec<u8>>();
    println!("{:?}", flat_image.as_ptr());

    let mut image = Image::from_flat(flat_image, 3, 3, BGR8);

    return image.to_nd_array(BGR8).unwrap();
}

fn image_show(frame: &ndarray::Array<u8, ndarray::Ix3>) {}

fn send_output(arrow_array: UnionArray) {
    let image = Image::from_arrow_array(arrow_array);

    println!("{:?}", image.as_ptr());
}

fn main() {
    /*
    // Read OpenCV Camera, default is nd_array BGR8
    let frame = camera_read();

    let mut image = Image::from_nd_array(frame, BGR8);

    // Convert to RGB8 to plot it with OpenCV imshow
    let frame = image.to_nd_array(RGB8).unwrap();

    image_show(&frame);

    // Convert it back to arrow to send it to dataflow
    let image = Image::from_nd_array(frame, BGR8);
    let arrow_array = image.to_arrow_array().unwrap();

    send_output(arrow_array);
    */
    let flat_image = (1..28).collect::<Vec<u8>>();
    println!("{:?}", flat_image.as_ptr());

    let mut builder = UInt8Array::builder(28);
    builder.append_slice(&flat_image);
    let arrow_array = builder.finish();
    println!("{:?}", arrow_array.values().as_ptr());

}