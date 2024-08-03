use super::{Image, ImageData};

use eyre::{Context, Report, Result};

impl Image {
    pub fn new_rgb8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Result<Self> {
        if pixels.len() != (width * height * 3) as usize {
            return Err(Report::msg("Invalid pixel data length."));
        }

        Ok(Self::ImageRGB8(ImageData {
            pixels,
            width,
            height,
            name: name.map(|s| s.to_string()),
        }))
    }

    pub fn rgb8_from_ndarray(
        array: ndarray::Array<u8, ndarray::Ix3>,
        name: Option<&str>,
    ) -> Result<Self> {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let pixels = array.into_raw_vec();

        Self::new_rgb8(pixels, width, height, name)
    }

    pub fn rgb8_to_ndarray(self) -> Result<ndarray::Array<u8, ndarray::Ix3>> {
        match self {
                Self::ImageRGB8(image) => ndarray::Array::from_shape_vec(
                    (image.height as usize, image.width as usize, 3),
                    image.pixels,
                )
                .wrap_err("Failed to reshape pixels into ndarray: width, height and RGB8 encoding doesn't match pixels data length."),
                _ => Err(Report::msg("Image is not in RGB8 format")),
            }
    }

    pub fn rgb8_to_ndarray_view(&self) -> Result<ndarray::ArrayView<u8, ndarray::Ix3>> {
        match self {
                Self::ImageRGB8(image) => ndarray::ArrayView::from_shape(
                    (image.height as usize, image.width as usize, 3),
                    &image.pixels,
                )
                .wrap_err("Failed to reshape pixels into ndarray: width, height and RGB8 encoding doesn't match pixels data length."),
                _ => Err(Report::msg("Image is not in RGB8 format")),
            }
    }

    pub fn rgb8_to_ndarray_view_mut(&mut self) -> Result<ndarray::ArrayViewMut<u8, ndarray::Ix3>> {
        match self {
                Self::ImageRGB8(image) => ndarray::ArrayViewMut::from_shape(
                    (image.height as usize, image.width as usize, 3),
                    &mut image.pixels,
                )
                .wrap_err("Failed to reshape pixels into ndarray: width, height and RGB8 encoding doesn't match pixels data length."),
                _ => Err(Report::msg("Image is not in RGB8 format")),
            }
    }
}

mod tests {
    #[test]
    fn test_rgb8_creation() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();

        Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();
    }

    #[test]
    fn test_rgb8_from_ndarray() {
        use ndarray::Array3;

        use crate::image::Image;

        let array = Array3::<u8>::zeros((3, 3, 3));

        Image::rgb8_from_ndarray(array, Some("camera.test")).unwrap();
    }

    #[test]
    fn test_rgb8_to_ndarray() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();

        let image = Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.rgb8_to_ndarray().unwrap();
    }

    #[test]
    fn test_rgb8_to_ndarray_view() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();

        let image = Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.rgb8_to_ndarray_view().unwrap();
    }

    #[test]
    fn test_rgb8_to_ndarray_view_mut() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();

        let mut image = Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.rgb8_to_ndarray_view_mut().unwrap();
    }

    #[test]
    fn test_rgb8_ndarray_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();
        let original_buffer_address = flat_image.as_ptr();

        let rgb8_image = Image::new_rgb8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = rgb8_image.as_ptr();

        let rgb8_ndarray = rgb8_image.rgb8_to_ndarray().unwrap();
        let ndarray_buffer_address = rgb8_ndarray.as_ptr();

        let final_image = Image::rgb8_from_ndarray(rgb8_ndarray, None).unwrap();
        let final_image_buffer_address = final_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, ndarray_buffer_address);
        assert_eq!(ndarray_buffer_address, final_image_buffer_address);
    }
}
