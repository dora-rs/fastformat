use crate::arrow::{column_by_name, union_field, union_lookup_table};

use super::{container::DataContainer, encoding::Encoding, Image};
use eyre::{Context, Report, Result};

use std::{collections::HashMap, mem, sync::Arc};

impl Image {
    unsafe fn arrow_data_to_vec<T: arrow::datatypes::ArrowPrimitiveType, G>(
        array: &arrow::array::UnionArray,
        lookup_table: &HashMap<String, i8>,
    ) -> Result<Vec<G>> {
        let arrow = column_by_name::<arrow::array::PrimitiveArray<T>>(array, "data", lookup_table)?;
        let ptr = arrow.values().as_ptr();
        let len = arrow.len();

        Ok(Vec::from_raw_parts(ptr as *mut G, len, len))
    }

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
    /// let array = image.to_arrow().unwrap();
    ///
    /// let image = Image::from_arrow(array).unwrap();
    /// ```
    pub fn from_arrow(array: arrow::array::UnionArray) -> Result<Self> {
        use arrow::array::Array;

        let union_fields = match array.data_type() {
            arrow::datatypes::DataType::Union(fields, ..) => fields,
            _ => {
                return Err(Report::msg("UnionArray has invalid data type."));
            }
        };

        let lookup_table = union_lookup_table(union_fields);

        let width =
            column_by_name::<arrow::array::UInt32Array>(&array, "width", &lookup_table)?.value(0);
        let height =
            column_by_name::<arrow::array::UInt32Array>(&array, "height", &lookup_table)?.value(0);
        let encoding = Encoding::from_string(
            column_by_name::<arrow::array::StringArray>(&array, "encoding", &lookup_table)?
                .value(0)
                .to_string(),
        )?;

        let name = column_by_name::<arrow::array::StringArray>(&array, "name", &lookup_table)?;

        let name = if name.is_null(0) {
            None
        } else {
            Some(name.value(0).to_string())
        };

        unsafe {
            let array = mem::ManuallyDrop::new(array);

            let data = match encoding {
                Encoding::RGB8 => DataContainer::from_u8(Self::arrow_data_to_vec::<
                    arrow::datatypes::UInt8Type,
                    u8,
                >(&array, &lookup_table)?),
                Encoding::BGR8 => DataContainer::from_u8(Self::arrow_data_to_vec::<
                    arrow::datatypes::UInt8Type,
                    u8,
                >(&array, &lookup_table)?),
                Encoding::GRAY8 => DataContainer::from_u8(Self::arrow_data_to_vec::<
                    arrow::datatypes::UInt8Type,
                    u8,
                >(&array, &lookup_table)?),
            };

            Ok(Image {
                data,
                width,
                height,
                encoding,
                name,
            })
        }
    }

    fn convert_image_details_to_arrow(image: Image) -> Result<Vec<Arc<dyn arrow::array::Array>>> {
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
    /// let arrow_array = image.to_arrow().unwrap();
    /// ```
    pub fn to_arrow(self) -> Result<arrow::array::UnionArray> {
        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

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

        let children = Self::convert_image_details_to_arrow(self)?;

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

        let arrow_image = bgr8_image.to_arrow().unwrap();

        let new_image = Image::from_arrow(arrow_image).unwrap();
        let final_image_buffer = new_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, final_image_buffer);
    }
}
