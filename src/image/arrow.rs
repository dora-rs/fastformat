use super::{data::ImageData, encoding::Encoding, Image};
use arrow::array::Array;
use eyre::{Context, ContextCompat, Report, Result};

use std::collections::HashMap;

use std::sync::Arc;

use crate::arrow::{
    array_data_to_map, primitive_array_from_raw_parts, primitive_array_view_from_raw_parts,
    primitive_buffer_from_map, primitive_singleton_from_raw_parts, utf8_buffer_from_map,
    utf8_singleton_from_raw_parts,
};

impl<'a> Image<'a> {
    pub fn raw_parts(
        array_data: arrow::array::ArrayData,
    ) -> Result<
        HashMap<
            String,
            (
                arrow::buffer::Buffer,
                Option<arrow::buffer::OffsetBuffer<i32>>,
            ),
        >,
    > {
        let mut map = array_data_to_map(array_data)?;

        let mut result = HashMap::new();

        result.insert(
            "width".to_string(),
            primitive_buffer_from_map::<arrow::datatypes::UInt32Type>("width", &mut map)?,
        );

        result.insert(
            "height".to_string(),
            primitive_buffer_from_map::<arrow::datatypes::UInt32Type>("height", &mut map)?,
        );

        result.insert(
            "encoding".to_string(),
            utf8_buffer_from_map("encoding", &mut map)?,
        );

        result.insert("name".to_string(), utf8_buffer_from_map("name", &mut map)?);

        let encoding = Encoding::from_string(utf8_singleton_from_raw_parts("encoding", &result)?)?;
        let data = match encoding {
            Encoding::RGB8 => {
                primitive_buffer_from_map::<arrow::datatypes::UInt8Type>("data", &mut map)?
            }
            Encoding::BGR8 => {
                primitive_buffer_from_map::<arrow::datatypes::UInt8Type>("data", &mut map)?
            }
            Encoding::GRAY8 => {
                primitive_buffer_from_map::<arrow::datatypes::UInt8Type>("data", &mut map)?
            }
        };

        result.insert("data".to_string(), data);

        Ok(result)
    }

    pub fn from_raw_parts(
        mut raw_parts: HashMap<
            String,
            (
                arrow::buffer::Buffer,
                Option<arrow::buffer::OffsetBuffer<i32>>,
            ),
        >,
    ) -> Result<Self> {
        let width = primitive_singleton_from_raw_parts::<arrow::datatypes::UInt32Type>(
            "width", &raw_parts,
        )?;

        let height = primitive_singleton_from_raw_parts::<arrow::datatypes::UInt32Type>(
            "height", &raw_parts,
        )?;

        let encoding =
            Encoding::from_string(utf8_singleton_from_raw_parts("encoding", &raw_parts)?)?;

        let name = Some(utf8_singleton_from_raw_parts("name", &raw_parts)?);

        let data = match encoding {
            Encoding::RGB8 => primitive_array_from_raw_parts::<arrow::datatypes::UInt8Type>(
                "data",
                &mut raw_parts,
            )?,
            Encoding::BGR8 => primitive_array_from_raw_parts::<arrow::datatypes::UInt8Type>(
                "data",
                &mut raw_parts,
            )?,
            Encoding::GRAY8 => primitive_array_from_raw_parts::<arrow::datatypes::UInt8Type>(
                "data",
                &mut raw_parts,
            )?,
        };

        Ok(Self {
            data: ImageData::from_vec_u8(data),
            width,
            height,
            encoding,
            name,
        })
    }

    pub fn view_from_raw_parts(
        raw_parts: &'a mut HashMap<
            String,
            (
                arrow::buffer::Buffer,
                Option<arrow::buffer::OffsetBuffer<i32>>,
            ),
        >,
    ) -> Result<Self> {
        let width = primitive_singleton_from_raw_parts::<arrow::datatypes::UInt32Type>(
            "width", &raw_parts,
        )?;

        let height = primitive_singleton_from_raw_parts::<arrow::datatypes::UInt32Type>(
            "height", &raw_parts,
        )?;

        let encoding =
            Encoding::from_string(utf8_singleton_from_raw_parts("encoding", &raw_parts)?)?;

        let name = Some(utf8_singleton_from_raw_parts("name", &raw_parts)?);

        let data = match encoding {
            Encoding::RGB8 => primitive_array_view_from_raw_parts::<arrow::datatypes::UInt8Type>(
                "data", raw_parts,
            )?,
            Encoding::BGR8 => primitive_array_view_from_raw_parts::<arrow::datatypes::UInt8Type>(
                "data", raw_parts,
            )?,
            Encoding::GRAY8 => primitive_array_view_from_raw_parts::<arrow::datatypes::UInt8Type>(
                "data", raw_parts,
            )?,
        };

        Ok(Self {
            data: ImageData::from_slice_u8(data),
            width,
            height,
            encoding,
            name,
        })
    }

    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_parts(Self::raw_parts(array_data)?)
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        let width = Arc::new(arrow::array::UInt32Array::from(vec![self.width; 1]));
        let height = Arc::new(arrow::array::UInt32Array::from(vec![self.height; 1]));

        let encoding = Arc::new(arrow::array::StringArray::from(vec![
            self.encoding
                .to_string();
            1
        ]));

        let name = Arc::new(arrow::array::StringArray::from(vec![self.name.clone(); 1]));

        let data: Arc<dyn arrow::array::Array> = match self.encoding {
            Encoding::RGB8 => Arc::new(arrow::array::UInt8Array::from(self.data.into_u8()?)),
            Encoding::BGR8 => Arc::new(arrow::array::UInt8Array::from(self.data.into_u8()?)),
            Encoding::GRAY8 => Arc::new(arrow::array::UInt8Array::from(self.data.into_u8()?)),
        };

        let children = vec![data, width, height, encoding, name];
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

        Ok(
            arrow::array::UnionArray::try_new(union_fields, type_ids, Some(offsets), children)
                .wrap_err("Failed to create UnionArray with Image data.")?
                .into_data(),
        )
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

        println!("{:?}", arrow_image);

        let new_image = Image::from_arrow(arrow_image).unwrap();
        let final_image_buffer = new_image.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, final_image_buffer);
    }
}
