use super::{Image, ImageData};

use eyre::{Context, Report, Result};

impl Image {
    pub fn new_gray8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Result<Self> {
        if pixels.len() != (width * height) as usize {
            return Err(Report::msg("Invalid pixels data length."));
        }

        Ok(Self::ImageGray8(ImageData {
            pixels,
            width,
            height,
            name: name.map(|s| s.to_string()),
        }))
    }

    pub fn gray8_from_ndarray(
        array: ndarray::Array<u8, ndarray::Ix2>,
        name: Option<&str>,
    ) -> Result<Self> {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let pixels = array.into_raw_vec();

        Self::new_gray8(pixels, width, height, name)
    }

    pub fn gray8_to_ndarray(self) -> Result<ndarray::Array<u8, ndarray::Ix2>> {
        match self {
                Self::ImageGray8(image) => ndarray::Array::from_shape_vec(
                    (image.height as usize, image.width as usize),
                    image.pixels,
                )
                .wrap_err("Failed to reshape pixels into ndarray: width, height and Gray8 encoding doesn't match pixels data length."),
                _ => Err(Report::msg("Image is not in Gray8 format")),
            }
    }

    pub fn gray8_to_ndarray_view(&self) -> Result<ndarray::ArrayView<u8, ndarray::Ix2>> {
        match self {
                Self::ImageGray8(image) => ndarray::ArrayView::from_shape(
                    (image.height as usize, image.width as usize),
                    &image.pixels,
                )
                .wrap_err("Failed to reshape pixels into ndarray: width, height and Gray8 encoding doesn't match pixels data length."),
                _ => Err(Report::msg("Image is not in Gray8 format")),
            }
    }

    pub fn gray8_to_ndarray_view_mut(&mut self) -> Result<ndarray::ArrayViewMut<u8, ndarray::Ix2>> {
        match self {
                Self::ImageGray8(image) => ndarray::ArrayViewMut::from_shape(
                    (image.height as usize, image.width as usize),
                    &mut image.pixels,
                )
                .wrap_err("Failed to reshape pixels into ndarray: width, height and Gray8 encoding doesn't match pixels data length."),
                _ => Err(Report::msg("Image is not in Gray8 format")),
            }
    }
}

mod test {
    #[test]
    fn test_gray8_creation() {
        use crate::image::Image;

        let flat_image = (1..10).collect::<Vec<u8>>();

        Image::new_gray8(flat_image, 3, 3, Some("camera.test")).unwrap();
    }

    #[test]
    fn test_gray8_from_ndarray() {
        use ndarray::Array2;

        use crate::image::Image;

        let array = Array2::<u8>::zeros((3, 3));

        Image::gray8_from_ndarray(array, Some("camera.test")).unwrap();
    }

    #[test]
    fn test_gray8_to_ndarray() {
        use crate::image::Image;

        let flat_image = (1..10).collect::<Vec<u8>>();

        let image = Image::new_gray8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.gray8_to_ndarray().unwrap();
    }

    #[test]
    fn test_gray8_to_ndarray_view() {
        use crate::image::Image;

        let flat_image = (1..10).collect::<Vec<u8>>();

        let image = Image::new_gray8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.gray8_to_ndarray_view().unwrap();
    }

    #[test]
    fn test_gray8_to_ndarray_view_mut() {
        use crate::image::Image;

        let flat_image = (1..10).collect::<Vec<u8>>();

        let mut image = Image::new_gray8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.gray8_to_ndarray_view_mut().unwrap();
    }

    #[test]
    fn test_gray8_ndarray_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = (1..10).collect::<Vec<u8>>();
        let original_buffer_address = flat_image.as_ptr();

        let gray8_image = Image::new_gray8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = gray8_image.as_ptr();

        let gray8_ndarray = gray8_image.gray8_to_ndarray().unwrap();
        let ndarray_buffer_address = gray8_ndarray.as_ptr();

        let final_image = Image::gray8_from_ndarray(gray8_ndarray, None).unwrap();
        let final_image_buffer_address = final_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, ndarray_buffer_address);
        assert_eq!(ndarray_buffer_address, final_image_buffer_address);
    }
}
