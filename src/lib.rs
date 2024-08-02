use pyo3::prelude::*;

pub mod arrow;

pub mod image;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn fastformat(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

mod tests {
    #[test]
    fn test_zero_copy_image_conversion() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();
        let original_buffer_address = flat_image.as_ptr();

        let image = Image::new_bgr8(flat_image, 3, 3, None);

        let frame = image.to_nd_array().unwrap();
        let image = Image::from_bgr8_nd_array(frame, Some("camera.left"));

        let frame = image.to_rgb().to_nd_array().unwrap();
        let image = Image::from_rgb8_nd_array(frame, Some("camera.left"));

        let frame = image.to_bgr().to_arrow().unwrap();
        let image = Image::from_arrow(frame).unwrap();

        let final_buffer_address = image.as_ptr();

        assert_eq!(original_buffer_address, final_buffer_address);
    }
}
