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
    fn test_ndarray_conversion() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();
        let original_buffer_address = flat_image.as_ptr();

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None);
        let image_buffer_address = bgr8_image.as_ptr();

        let bgr8_ndarray = bgr8_image.to_bgr8_ndarray().unwrap();
        let ndarray_buffer_address = bgr8_ndarray.as_ptr();

        let final_image = Image::from_bgr8_ndarray(bgr8_ndarray, None);
        let final_image_buffer_address = final_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, ndarray_buffer_address);
        assert_eq!(ndarray_buffer_address, final_image_buffer_address);
    }

    #[test]
    fn test_arrow_conversion() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();
        let original_buffer_address = flat_image.as_ptr();

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None);
        let image_buffer_address = bgr8_image.as_ptr();

        let arrow_image = bgr8_image.to_arrow().unwrap();

        let new_image = Image::from_arrow(arrow_image).unwrap();
        let final_image_buffer = new_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, final_image_buffer);
    }
}
