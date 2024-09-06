use super::{data::ImageData, encoding::Encoding, Image};
use eyre::{Report, Result};

impl Image<'_> {
    /// Creates a new `Image` in Gray8 format.
    ///
    /// This function constructs a new `Image` object with the given pixel data, width, height,
    /// and an optional name. It ensures that the pixel data length matches the expected size
    /// for the given width and height.
    ///
    /// # Arguments
    ///
    /// * `data` - A `Vec<u8>` containing the pixel data in Gray8 format.
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
    /// use fastformat_datatypes::image::Image;
    ///
    /// let data = vec![0; 9]; // 3x3 image with 1 byte per pixel
    /// let image = Image::new_gray8(data, 3, 3, Some("example")).unwrap();
    /// ```
    pub fn new_gray8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Result<Self> {
        if data.len() != (width * height) as usize {
            return Err(Report::msg("Invalid data data length."));
        }

        Ok(Image {
            data: ImageData::from_vec_u8(data),
            width,
            height,
            encoding: Encoding::GRAY8,
            name: name.map(|s| s.to_string()),
        })
    }
}

mod test {
    #[test]
    fn test_gray8_creation() {
        use crate::image::Image;

        let flat_image = (1..10).collect::<Vec<u8>>();

        Image::new_gray8(flat_image, 3, 3, Some("camera.test")).unwrap();
    }
}
