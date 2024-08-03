use crate::arrow::{column_by_name, union_field, union_look_up_table};

use super::{Image, ImageData};
use eyre::{Context, Report, Result};

use std::{mem, sync::Arc};

impl Image {
    pub fn from_arrow(array: arrow::array::UnionArray) -> Result<Self> {
        use arrow::array::Array;

        let union_fields = match array.data_type() {
            arrow::datatypes::DataType::Union(fields, ..) => fields,
            _ => {
                return Err(Report::msg("UnionArray has invalid data type."));
            }
        };

        let look_up_table = union_look_up_table(&union_fields);

        let width =
            column_by_name::<arrow::array::UInt32Array>(&array, "width", &look_up_table)?.value(0);
        let height =
            column_by_name::<arrow::array::UInt32Array>(&array, "height", &look_up_table)?.value(0);
        let encoding =
            column_by_name::<arrow::array::StringArray>(&array, "encoding", &look_up_table)?
                .value(0)
                .to_string();

        let name = column_by_name::<arrow::array::StringArray>(&array, "name", &look_up_table)?;

        let name = if name.is_null(0) {
            None
        } else {
            Some(name.value(0).to_string())
        };

        let name = name.as_ref().map(|s| s.as_str());

        unsafe {
            let array = mem::ManuallyDrop::new(array);
            let pixels = match encoding.as_str() {
                "RGB8" => {
                    column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table)?
                }
                "BGR8" => {
                    column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table)?
                }
                "GRAY8" => {
                    column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table)?
                }
                _ => {
                    return Err(Report::msg(format!("Invalid encoding: {}", encoding)));
                }
            };

            let ptr = pixels.values().as_ptr();
            let len = pixels.len();

            let pixels = Vec::from_raw_parts(ptr as *mut u8, len, len);

            return match encoding.as_str() {
                "RGB8" => Self::new_rgb8(pixels, width, height, name),
                "BGR8" => Self::new_bgr8(pixels, width, height, name),
                "GRAY8" => Self::new_gray8(pixels, width, height, name),
                _ => Err(Report::msg(format!("Invalid encoding: {}", encoding))),
            };
        }
    }

    fn get_image_details<T>(
        image: &ImageData<T>,
    ) -> (
        arrow::array::UInt32Array,
        arrow::array::UInt32Array,
        arrow::array::StringArray,
    ) {
        let width = arrow::array::UInt32Array::from(vec![image.width; 1]);
        let height = arrow::array::UInt32Array::from(vec![image.height; 1]);

        let name = arrow::array::StringArray::from(vec![image.name.clone(); 1]);

        (width, height, name)
    }

    pub fn to_arrow(self) -> Result<arrow::array::UnionArray> {
        let ((width, height, name), encoding, pixels, datatype) = match self {
            Image::ImageBGR8(image) => (
                Self::get_image_details(&image),
                arrow::array::StringArray::from(vec!["BGR8".to_string(); 1]),
                arrow::array::UInt8Array::from(image.pixels),
                arrow::datatypes::DataType::UInt8,
            ),
            Image::ImageRGB8(image) => (
                Self::get_image_details(&image),
                arrow::array::StringArray::from(vec!["RGB8".to_string(); 1]),
                arrow::array::UInt8Array::from(image.pixels),
                arrow::datatypes::DataType::UInt8,
            ),
            Image::ImageGray8(image) => (
                Self::get_image_details(&image),
                arrow::array::StringArray::from(vec!["GRAY8".to_string(); 1]),
                arrow::array::UInt8Array::from(image.pixels),
                arrow::datatypes::DataType::UInt8,
            ),
        };

        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

        let union_fields = [
            union_field(0, "pixels", datatype, false),
            union_field(1, "width", arrow::datatypes::DataType::UInt32, false),
            union_field(2, "height", arrow::datatypes::DataType::UInt32, false),
            union_field(3, "encoding", arrow::datatypes::DataType::Utf8, false),
            union_field(4, "name", arrow::datatypes::DataType::Utf8, true),
        ]
        .into_iter()
        .collect::<arrow::datatypes::UnionFields>();

        let children: Vec<Arc<dyn arrow::array::Array>> = vec![
            Arc::new(pixels),
            Arc::new(width),
            Arc::new(height),
            Arc::new(encoding),
            Arc::new(name),
        ];

        arrow::array::UnionArray::try_new(union_fields, type_ids, Some(offsets), children)
            .wrap_err("Failed to create UnionArray width Image data.")
    }
}

mod tests {
    #[test]
    fn test_arrow_conversion() {
        use crate::image::Image;

        let flat_image = (1..28).collect::<Vec<u8>>();
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
