use super::{data::ImageData, encoding::Encoding, Image};
use eyre::{Report, Result};

impl Image<'_> {
    /// Creates a new `Image` in RGB8 format.
    ///
    /// This function constructs a new `Image` object with the given pixel data, width, height,
    /// and an optional name. It ensures that the pixel data length matches the expected size
    /// for the given width and height.
    ///
    /// # Arguments
    ///
    /// * `data` - A `Vec<u8>` containing the pixel data in RGB8 format.
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
    /// let data = vec![0; 27]; // 3x3 image with 3 bytes per pixel
    /// let image = Image::new_rgb8(data, 3, 3, Some("example")).unwrap();
    /// ```
    pub fn new_rgb8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Result<Self> {
        if data.len() != (width * height * 3) as usize {
            return Err(Report::msg("Invalid pixel data length."));
        }

        Ok(Image {
            data: ImageData::from_vec_u8(data),
            width,
            height,
            encoding: Encoding::RGB8,
            name: name.map(|s| s.to_string()),
        })
    }
}

mod tests {
    #[test]
    fn test_rgb8_creation() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();
    }
}
