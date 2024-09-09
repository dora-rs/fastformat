use super::{data::ImageData, encoding::Encoding, Image};
use eyre::Result;
use fastformat_converter::arrow::{FastFormatArrowBuilder, FastFormatArrowRawData};

impl<'a> Image<'a> {
    /// Extracts raw data from an Arrow `ArrayData` and converts it to `FastFormatArrowRawData`.
    ///
    /// This function loads the primitive and UTF fields corresponding to an image's width, height,
    /// encoding, and name from the provided `ArrayData`. It determines the image encoding and loads
    /// the appropriate pixel data.
    ///
    /// # Arguments
    ///
    /// * `array_data` - The Arrow `ArrayData` containing the image metadata and pixel data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FastFormatArrowRawData` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the expected fields (width, height, encoding, or pixel data) are missing
    /// or if the data format is invalid.
    pub fn raw_data(array_data: arrow::array::ArrayData) -> Result<FastFormatArrowRawData> {
        use arrow::datatypes::{UInt32Type, UInt8Type};

        let raw_data = FastFormatArrowRawData::new(array_data)?
            .load_primitive::<UInt32Type>("width")?
            .load_primitive::<UInt32Type>("height")?
            .load_utf("encoding")?
            .load_utf("name")?;

        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;
        let raw_data = match encoding {
            Encoding::RGB8 => raw_data.load_primitive::<UInt8Type>("data")?,
            Encoding::BGR8 => raw_data.load_primitive::<UInt8Type>("data")?,
            Encoding::GRAY8 => raw_data.load_primitive::<UInt8Type>("data")?,
        };

        Ok(raw_data)
    }

    /// Constructs an `Image` object from `FastFormatArrowRawData`.
    ///
    /// This function parses the width, height, encoding, and name of the image from the
    /// provided raw data and loads the pixel data based on the encoding type.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - The raw data containing the image metadata and pixel information.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `Image` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the raw data is missing fields or contains invalid values.
    pub fn from_raw_data(mut raw_data: FastFormatArrowRawData) -> Result<Self> {
        use arrow::datatypes::{UInt32Type, UInt8Type};

        let width = raw_data.primitive_singleton::<UInt32Type>("width")?;
        let height = raw_data.primitive_singleton::<UInt32Type>("height")?;
        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;
        let name = Some(raw_data.utf8_singleton("name")?).filter(|s| !s.is_empty());

        let data = match encoding {
            Encoding::RGB8 => raw_data.primitive_array::<UInt8Type>("data")?,
            Encoding::BGR8 => raw_data.primitive_array::<UInt8Type>("data")?,
            Encoding::GRAY8 => raw_data.primitive_array::<UInt8Type>("data")?,
        };

        Ok(Self {
            data: ImageData::from_vec_u8(data),
            width,
            height,
            encoding,
            name,
        })
    }

    /// Creates a read-only view of an `Image` from `FastFormatArrowRawData`.
    ///
    /// This function provides a zero-copy read-only view of the image data, allowing efficient
    /// access to the metadata and pixel data without copying the underlying memory.
    ///
    /// # Arguments
    ///
    /// * `raw_data` - A reference to the raw data containing the image metadata and pixel information.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Image` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the raw data is invalid or if required fields are missing.
    pub fn view_from_raw_data(raw_data: &'a FastFormatArrowRawData) -> Result<Self> {
        use arrow::datatypes::{UInt32Type, UInt8Type};

        let width = raw_data.primitive_singleton::<UInt32Type>("width")?;
        let height = raw_data.primitive_singleton::<UInt32Type>("height")?;
        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;
        let name = Some(raw_data.utf8_singleton("name")?).filter(|s| !s.is_empty());

        let data = match encoding {
            Encoding::RGB8 => raw_data.primitive_array_view::<UInt8Type>("data")?,
            Encoding::BGR8 => raw_data.primitive_array_view::<UInt8Type>("data")?,
            Encoding::GRAY8 => raw_data.primitive_array_view::<UInt8Type>("data")?,
        };

        Ok(Self {
            data: ImageData::from_slice_u8(data),
            width,
            height,
            encoding,
            name,
        })
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
    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_data(Self::raw_data(array_data)?)
    }

    /// Converts an `Image` into Arrow `ArrayData`.
    ///
    /// This function serializes the image metadata and pixel data into Arrow format, allowing
    /// the image to be stored or transmitted as Arrow `ArrayData`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the serialized `ArrayData` if successful, or an error otherwise.
    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::datatypes::{
            DataType::{UInt32, UInt8, Utf8},
            UInt32Type, UInt8Type,
        };

        let raw_data = FastFormatArrowBuilder::new()
            .push_primitive_singleton::<UInt32Type>("width", self.width, UInt32, false)
            .push_primitive_singleton::<UInt32Type>("height", self.height, UInt32, false)
            .push_utf_singleton("encoding", self.encoding.to_string(), Utf8, false)
            .push_utf_singleton(
                "name",
                self.name.map_or_else(|| "".to_string(), |s| s),
                Utf8,
                false,
            );

        let raw_data = match self.encoding {
            Encoding::RGB8 => raw_data.push_primitive_array::<UInt8Type>(
                "data",
                self.data.into_u8()?,
                UInt8,
                false,
            ),
            Encoding::BGR8 => raw_data.push_primitive_array::<UInt8Type>(
                "data",
                self.data.into_u8()?,
                UInt8,
                false,
            ),
            Encoding::GRAY8 => raw_data.push_primitive_array::<UInt8Type>(
                "data",
                self.data.into_u8()?,
                UInt8,
                false,
            ),
        };

        raw_data.into_arrow()
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
