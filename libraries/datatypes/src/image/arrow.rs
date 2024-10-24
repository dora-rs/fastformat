use fastformat_converter::arrow::{
    builder::ArrowDataBuilder, consumer::ArrowDataConsumer, IntoArrow,
};

use super::{data::ImageData, encoding::Encoding, Image};

impl<'a> IntoArrow for Image<'a> {
    /// Converts an `Image` into Arrow `ArrayData`.
    ///
    /// This function serializes the image metadata and pixel data into Arrow format, allowing
    /// the image to be stored or transmitted as Arrow `ArrayData`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the serialized `ArrayData` if successful, or an error otherwise.
    fn into_arrow(self) -> eyre::Result<arrow::array::ArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<arrow::datatypes::UInt32Type>("width", self.width)
            .push_primitive_singleton::<arrow::datatypes::UInt32Type>("height", self.height)
            .push_utf8_singleton("encoding", self.encoding.to_string())
            .push_utf8_singleton(
                "name",
                match self.name {
                    Some(name) => name.to_owned(),
                    None => "".to_owned(),
                },
            );

        let builder = match self.encoding {
            Encoding::RGB8 | Encoding::BGR8 | Encoding::GRAY8 => builder
                .push_primitive_array::<arrow::datatypes::UInt8Type>("data", self.data.into_u8()?),
        };

        builder.build()
    }

    /// Converts Arrow `ArrayData` into an `Image`.
    ///
    /// This function combines the process of extracting raw data and converting it into an
    /// `Image` object.
    ///
    /// # Arguments
    ///
    /// * `array_data` - The Arrow `ArrayData` containing the image metadata and pixel data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Image` if successful, or an error otherwise.
    fn from_arrow(array_data: arrow::array::ArrayData) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let width = consumer.primitive_singleton::<arrow::datatypes::UInt32Type>("width")?;
        let height = consumer.primitive_singleton::<arrow::datatypes::UInt32Type>("height")?;
        let encoding = consumer.utf8_singleton("encoding")?;
        let name = consumer.utf8_singleton("name")?;

        let name = match name.as_str() {
            "" => None,
            _ => Some(name),
        };

        let encoding = Encoding::from_string(encoding)?;

        let data = match encoding {
            Encoding::RGB8 | Encoding::BGR8 | Encoding::GRAY8 => {
                consumer.primitive_array::<arrow::datatypes::UInt8Type>("data")?
            }
        };

        Ok(Self {
            width,
            height,
            encoding,
            name,
            data: ImageData::from_vec_u8(data),
        })
    }
}

mod tests {
    #[test]
    fn test_arrow_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let bgr8_image = Image::from_arrow(arrow_image).unwrap();
        let bgr8_image_buffer = bgr8_image.data.as_ptr();

        let rgb8_image = bgr8_image.into_rgb8().unwrap();
        let rgb8_image_buffer = rgb8_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, bgr8_image_buffer);
        assert_eq!(bgr8_image_buffer, rgb8_image_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_read_only() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let raw_data = Image::raw_data(arrow_image).unwrap();
        let new_image = Image::view_from_raw_data(&raw_data).unwrap();

        let final_image_buffer = new_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, final_image_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_copy_on_write() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let raw_data = Image::raw_data(arrow_image).unwrap();
        let bgr8_image = Image::view_from_raw_data(&raw_data).unwrap();
        let rgb8_image = bgr8_image.into_rgb8().unwrap();

        let final_image_buffer = rgb8_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_ne!(image_buffer_address, final_image_buffer);
    }
}
