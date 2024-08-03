use super::{Image, ImageData};

use eyre::{Context, Report, Result};

impl Image {
    /// Creates a new `Image` in Gray8 format.
    ///
    /// This function constructs a new `Image` object with the given pixel data, width, height,
    /// and an optional name. It ensures that the pixel data length matches the expected size
    /// for the given width and height.
    ///
    /// # Arguments
    ///
    /// * `pixels` - A `Vec<u8>` containing the pixel data in Gray8 format.
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `name` - An optional string slice representing the name of the image.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `Image` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of the pixel data does not match the expected size
    /// based on the width and height.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let pixels = vec![0; 9]; // 3x3 image with 1 byte per pixel
    /// let image = Image::new_gray8(pixels, 3, 3, Some("example")).unwrap();
    /// ```
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

    /// Creates a new `Image` in Gray8 format from an ndarray.
    ///
    /// This function constructs a new `Image` object from an `ndarray::Array` with shape (height, width).
    /// It converts the ndarray into a raw vector and uses it to create the `Image`.
    ///
    /// # Arguments
    ///
    /// * `array` - An `ndarray::Array<u8, ndarray::Ix2>` containing the pixel data in Gray8 format.
    /// * `name` - An optional string slice representing the name of the image.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `Image` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the ndarray cannot be converted into a valid `Image`.
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::Array2;
    /// use fastformat::image::Image;
    ///
    /// let array = Array2::<u8>::zeros((3, 3)); // 3x3 image
    /// let image = Image::gray8_from_ndarray(array, Some("example")).unwrap();
    /// ```
    pub fn gray8_from_ndarray(
        array: ndarray::Array<u8, ndarray::Ix2>,
        name: Option<&str>,
    ) -> Result<Self> {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let pixels = array.into_raw_vec();

        Self::new_gray8(pixels, width, height, name)
    }

    /// Converts a Gray8 `Image` into an ndarray.
    ///
    /// This function takes a Gray8 `Image` and converts it into an `ndarray::Array<u8, ndarray::Ix2>`.
    /// The resulting ndarray has shape (height, width).
    ///
    /// # Arguments
    ///
    /// * `self` - The `Image` object to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed ndarray if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Image` is not in Gray8 format or if the pixel data cannot be reshaped
    /// into the expected ndarray format.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let pixels = vec![0; 9]; // 3x3 image with 1 byte per pixel
    /// let image = Image::new_gray8(pixels, 3, 3, Some("example")).unwrap();
    ///
    /// let ndarray = image.gray8_to_ndarray().unwrap();
    /// ```
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

    /// Converts a Gray8 `Image` into an ndarray view.
    ///
    /// This function takes a reference to a Gray8 `Image` and creates an `ndarray::ArrayView<u8, ndarray::Ix2>`
    /// over the pixel data. The resulting view has shape (height, width).
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the `Image` object to be viewed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the ndarray view if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Image` is not in Gray8 format or if the pixel data cannot be reshaped
    /// into the expected ndarray view format.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let pixels = vec![0; 9]; // 3x3 image with 1 byte per pixel
    /// let image = Image::new_gray8(pixels, 3, 3, Some("example")).unwrap();
    ///
    /// let ndarray_view = image.gray8_to_ndarray_view().unwrap();
    /// ```
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

    /// Converts a mutable Gray8 `Image` into a mutable ndarray view.
    ///
    /// This function takes a mutable reference to a Gray8 `Image` and creates an `ndarray::ArrayViewMut<u8, ndarray::Ix2>`
    /// over the pixel data. The resulting view has shape (height, width).
    ///
    /// # Arguments
    ///
    /// * `&mut self` - A mutable reference to the `Image` object to be viewed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the mutable ndarray view if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the `Image` is not in Gray8 format or if the pixel data cannot be reshaped
    /// into the expected mutable ndarray view format.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let pixels = vec![0; 9]; // 3x3 image with 1 byte per pixel
    /// let mut image = Image::new_gray8(pixels, 3, 3, Some("example")).unwrap();
    ///
    /// let ndarray_view_mut = image.gray8_to_ndarray_view_mut().unwrap();
    /// ```
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
