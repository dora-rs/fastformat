use crate::arrow::{arrow_union_into_map, get_primitive_array_from_map, get_utf8_array_from_map};

use super::{container::DataContainer, encoding::Encoding, Image};
use eyre::{Context, ContextCompat, Report, Result};

use std::sync::Arc;

impl Image {
    /// Constructs an `Image` from an Arrow `UnionArray`.
    ///
    /// This function takes an Arrow `UnionArray` and extracts the necessary fields to construct
    /// an `Image` object. It validates the data type of the `UnionArray`, builds a lookup table for
    /// the fields, retrieves the image properties (width, height, encoding, name), and decodes the
    /// pixel data based on the encoding.
    ///
    /// # Arguments
    ///
    /// * `array` - A reference to an `arrow::array::UnionArray` that contains the image data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `Image` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the `UnionArray` has an invalid data type, if required fields are missing,
    /// or if the pixel data cannot be downcasted to the expected type based on the encoding.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let data = vec![0; 27]; // 3x3 image with 3 bytes per pixel
    /// let image = Image::new_bgr8(data, 3, 3, None).unwrap();
    /// let array = image.into_arrow().unwrap();
    ///
    /// let image = Image::from_arrow(array).unwrap();
    /// ```
    pub fn from_arrow(array: arrow::array::UnionArray) -> Result<Self> {
        let mut map = arrow_union_into_map(array)?;

        let width =
            get_primitive_array_from_map::<u32, arrow::datatypes::UInt32Type>("width", &mut map)?
                .first()
                .cloned()
                .wrap_err(Report::msg("width field must contains at least 1 value!"))?;

        let height =
            get_primitive_array_from_map::<u32, arrow::datatypes::UInt32Type>("height", &mut map)?
                .first()
                .cloned()
                .wrap_err(Report::msg("height field must contains at least 1 value!"))?;

        let encoding = Encoding::from_string(
            get_utf8_array_from_map("encoding", &mut map)?
                .first()
                .cloned()
                .wrap_err(Report::msg(
                    "encoding field must contains at least 1 value!",
                ))?,
        )?;

        let name = get_utf8_array_from_map("name", &mut map)?
            .first()
            .cloned()
            .wrap_err(Report::msg("name field must contains at least 1 value!"))?;

        let name = match name.as_str() {
            "" => None,
            _ => Some(name),
        };

        let data = match encoding {
            Encoding::RGB8 => {
                let data = get_primitive_array_from_map::<u8, arrow::datatypes::UInt8Type>(
                    "data", &mut map,
                )?;

                DataContainer::from_u8(data)
            }
            Encoding::BGR8 => {
                let data = get_primitive_array_from_map::<u8, arrow::datatypes::UInt8Type>(
                    "data", &mut map,
                )?;

                DataContainer::from_u8(data)
            }
            Encoding::GRAY8 => {
                let data = get_primitive_array_from_map::<u8, arrow::datatypes::UInt8Type>(
                    "data", &mut map,
                )?;

                DataContainer::from_u8(data)
            }
        };

        Ok(Image {
            data,
            width,
            height,
            encoding,
            name,
        })
    }

    fn convert_image_details_into_arrow(image: Image) -> Result<Vec<Arc<dyn arrow::array::Array>>> {
        let width = Arc::new(arrow::array::UInt32Array::from(vec![image.width; 1]));
        let height = Arc::new(arrow::array::UInt32Array::from(vec![image.height; 1]));

        let encoding = Arc::new(arrow::array::StringArray::from(vec![
            image
                .encoding
                .to_string();
            1
        ]));

        let name = Arc::new(arrow::array::StringArray::from(vec![image.name.clone(); 1]));

        let data: Arc<dyn arrow::array::Array> = match image.encoding {
            Encoding::RGB8 => Arc::new(arrow::array::UInt8Array::from(image.data.into_u8()?)),
            Encoding::BGR8 => Arc::new(arrow::array::UInt8Array::from(image.data.into_u8()?)),
            Encoding::GRAY8 => Arc::new(arrow::array::UInt8Array::from(image.data.into_u8()?)),
        };

        Ok(vec![data, width, height, encoding, name])
    }

    /// Converts an `Image` into an Arrow `UnionArray`.
    ///
    /// This function takes an `Image` object and converts it into an Arrow `UnionArray`
    /// that contains the image properties and pixel data. The conversion handles different
    /// image encodings (BGR8, RGB8, GRAY8) and ensures that the resulting `UnionArray`
    /// contains all necessary fields.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Image` object to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `arrow::array::UnionArray` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the `UnionArray` cannot be created due to issues with the provided data.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::image::Image;
    ///
    /// let data = vec![0; 640 * 480 * 3];
    /// let image = Image::new_bgr8(data, 640, 480, None).unwrap();
    ///
    /// let arrow_array = image.into_arrow().unwrap();
    /// ```
    pub fn into_arrow(self) -> Result<arrow::array::UnionArray> {
        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

        fn union_field(
            index: i8,
            name: &str,
            data_type: arrow::datatypes::DataType,
            nullable: bool,
        ) -> (i8, Arc<arrow::datatypes::Field>) {
            (
                index,
                Arc::new(arrow::datatypes::Field::new(name, data_type, nullable)),
            )
        }

        let datatype = match self.encoding {
            Encoding::RGB8 => arrow::datatypes::DataType::UInt8,
            Encoding::BGR8 => arrow::datatypes::DataType::UInt8,
            Encoding::GRAY8 => arrow::datatypes::DataType::UInt8,
        };

        let union_fields = [
            union_field(0, "data", datatype, false),
            union_field(1, "width", arrow::datatypes::DataType::UInt32, false),
            union_field(2, "height", arrow::datatypes::DataType::UInt32, false),
            union_field(3, "encoding", arrow::datatypes::DataType::Utf8, false),
            union_field(4, "name", arrow::datatypes::DataType::Utf8, true),
        ]
        .into_iter()
        .collect::<arrow::datatypes::UnionFields>();

        let children = Self::convert_image_details_into_arrow(self)?;

        arrow::array::UnionArray::try_new(union_fields, type_ids, Some(offsets), children)
            .wrap_err("Failed to create UnionArray with Image data.")
    }
}

mod tests {
    #[test]
    fn test_arrow_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr();

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let new_image = Image::from_arrow(arrow_image).unwrap();
        let final_image_buffer = new_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, final_image_buffer);
    }
}
