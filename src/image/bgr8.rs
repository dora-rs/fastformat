use super::{Image, ImageData};

use eyre::{Context, Report, Result};

impl Image {
    /// Creates a new `Image` in BGR8 format.
    ///
    /// This function constructs a new `Image` object with the given pixel data, width, height,
    /// and an optional name. It ensures that the pixel data length matches the expected size
    /// for the given width, height, and BGR8 encoding (3 bytes per pixel).
    ///
    /// # Arguments
    ///
    /// * `data` - A `Vec<u8>` containing the pixel data in BGR8 format.
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
    /// based on the width, height, and BGR8 encoding.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let data = vec![0; 27]; // 3x3 image with 3 bytes per pixel
    /// let image = Image::new_bgr8(data, 3, 3, Some("example")).unwrap();
    /// ```
    pub fn new_bgr8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Result<Self> {
        if width * height * 3 != data.len() as u32 {
            return Err(Report::msg(
                "Width, height and BGR8 encoding doesn't match data data length.",
            ));
        }

        Ok(Self::ImageBGR8(ImageData {
            data,
            width,
            height,
            name: name.map(|s| s.to_string()),
        }))
    }

    /// Creates a new `Image` in BGR8 format from an ndarray.
    ///
    /// This function constructs a new `Image` object from an `ndarray::Array` with shape (height, width, 3).
    /// It converts the ndarray into a raw vector and uses it to create the `Image`.
    ///
    /// # Arguments
    ///
    /// * `array` - An `ndarray::Array<u8, ndarray::Ix3>` containing the pixel data in BGR8 format.
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
    /// use ndarray::Array3;
    /// use fastformat::image::Image;
    ///
    /// let array = Array3::<u8>::zeros((3, 3, 3)); // 3x3 image with 3 channels
    /// let image = Image::bgr8_from_ndarray(array, Some("example")).unwrap();
    /// ```
    pub fn bgr8_from_ndarray(
        array: ndarray::Array<u8, ndarray::Ix3>,
        name: Option<&str>,
    ) -> Result<Self> {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let data = array.into_raw_vec();

        Self::new_bgr8(data, width, height, name)
    }

    /// Converts a BGR8 `Image` into an ndarray.
    ///
    /// This function takes a BGR8 `Image` and converts it into an `ndarray::Array<u8, ndarray::Ix3>`.
    /// The resulting ndarray has shape (height, width, 3).
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
    /// Returns an error if the `Image` is not in BGR8 format or if the pixel data cannot be reshaped
    /// into the expected ndarray format.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let data = vec![0; 27]; // 3x3 image with 3 bytes per pixel
    /// let image = Image::new_bgr8(data, 3, 3, Some("example")).unwrap();
    ///
    /// let ndarray = image.bgr8_to_ndarray().unwrap();
    /// ```
    pub fn bgr8_to_ndarray(self) -> Result<ndarray::Array<u8, ndarray::Ix3>> {
        match self {
                Self::ImageBGR8(image) => ndarray::Array::from_shape_vec(
                    (image.height as usize, image.width as usize, 3),
                    image.data,
                )
                .wrap_err("Failed to reshape data into ndarray: width, height and BGR8 encoding doesn't match data data length."),
                _ => Err(Report::msg("Image is not in BGR8 format")),
            }
    }

    /// Converts a BGR8 `Image` into an ndarray view.
    ///
    /// This function takes a reference to a BGR8 `Image` and creates an `ndarray::ArrayView<u8, ndarray::Ix3>`
    /// over the pixel data. The resulting view has shape (height, width, 3).
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
    /// Returns an error if the `Image` is not in BGR8 format or if the pixel data cannot be reshaped
    /// into the expected ndarray view format.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let data = vec![0; 27]; // 3x3 image with 3 bytes per pixel
    /// let image = Image::new_bgr8(data, 3, 3, Some("example")).unwrap();
    ///
    /// let ndarray_view = image.bgr8_to_ndarray_view().unwrap();
    /// ```
    pub fn bgr8_to_ndarray_view(&self) -> Result<ndarray::ArrayView<u8, ndarray::Ix3>> {
        match self {
                Self::ImageBGR8(image) => ndarray::ArrayView::from_shape(
                    (image.height as usize, image.width as usize, 3),
                    &image.data,
                )
                .wrap_err("Failed to reshape data into ndarray: width, height and BGR8 encoding doesn't match data data length."),
                _ => Err(Report::msg("Image is not in BGR8 format")),
            }
    }

    /// Converts a mutable BGR8 `Image` into a mutable ndarray view.
    ///
    /// This function takes a mutable reference to a BGR8 `Image` and creates an `ndarray::ArrayViewMut<u8, ndarray::Ix3>`
    /// over the pixel data. The resulting view has shape (height, width, 3).
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
    /// Returns an error if the `Image` is not in BGR8 format or if the pixel data cannot be reshaped
    /// into the expected mutable ndarray view format.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let data = vec![0; 27]; // 3x3 image with 3 bytes per pixel
    /// let mut image = Image::new_bgr8(data, 3, 3, Some("example")).unwrap();
    ///
    /// let ndarray_view_mut = image.bgr8_to_ndarray_view_mut().unwrap();
    /// ```
    pub fn bgr8_to_ndarray_view_mut(&mut self) -> Result<ndarray::ArrayViewMut<u8, ndarray::Ix3>> {
        match self {
                Self::ImageBGR8(image) => ndarray::ArrayViewMut::from_shape(
                    (image.height as usize, image.width as usize, 3),
                    &mut image.data,
                )
                .wrap_err("Failed to reshape data into ndarray: width, height and BGR8 encoding doesn't match data data length."),
                _ => Err(Report::msg("Image is not in BGR8 format")),
            }
    }
}

mod tests {
    #[test]
    fn test_bgr8_creation() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();
    }

    #[test]
    fn test_bgr8_from_ndarray() {
        use ndarray::Array3;

        use crate::image::Image;

        let array = Array3::<u8>::zeros((3, 3, 3));

        Image::bgr8_from_ndarray(array, Some("camera.test")).unwrap();
    }

    #[test]
    fn test_bgr8_to_ndarray() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.bgr8_to_ndarray().unwrap();
    }

    #[test]
    fn test_bgr8_to_ndarray_view() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.bgr8_to_ndarray_view().unwrap();
    }

    #[test]
    fn test_bgr8_to_ndarray_view_mut() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let mut image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.bgr8_to_ndarray_view_mut().unwrap();
    }

    #[test]
    fn test_bgr8_ndarray_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr();

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.as_ptr();

        let bgr8_ndarray = bgr8_image.bgr8_to_ndarray().unwrap();
        let ndarray_buffer_address = bgr8_ndarray.as_ptr();

        let final_image = Image::bgr8_from_ndarray(bgr8_ndarray, None).unwrap();
        let final_image_buffer_address = final_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, ndarray_buffer_address);
        assert_eq!(ndarray_buffer_address, final_image_buffer_address);
    }
}
