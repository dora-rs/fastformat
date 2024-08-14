use super::{encoding::Encoding, BBox};
use crate::arrow::{
    array_data_to_map, primitive_array_from_raw_parts, primitive_array_view_from_raw_parts,
    primitive_buffer_from_map, primitive_singleton_from_raw_parts, utf8_array_from_raw_parts,
    utf8_buffer_from_map, utf8_singleton_from_raw_parts,
};

use eyre::{Context, ContextCompat, Report, Result};

use std::borrow::Cow;
use std::{collections::HashMap, sync::Arc};

impl<'a> BBox<'a> {
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
            "data".to_string(),
            primitive_buffer_from_map::<arrow::datatypes::Float32Type>("data", &mut map)?,
        );

        result.insert(
            "confidence".to_string(),
            primitive_buffer_from_map::<arrow::datatypes::Float32Type>("confidence", &mut map)?,
        );

        result.insert(
            "label".to_string(),
            utf8_buffer_from_map("label", &mut map)?,
        );

        result.insert(
            "encoding".to_string(),
            utf8_buffer_from_map("encoding", &mut map)?,
        );

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
        let data = primitive_array_from_raw_parts::<arrow::datatypes::Float32Type>(
            "data",
            &mut raw_parts,
        )?;

        let confidence = primitive_array_from_raw_parts::<arrow::datatypes::Float32Type>(
            "confidence",
            &mut raw_parts,
        )?;

        let label = utf8_array_from_raw_parts("label", &mut raw_parts)?;

        let encoding =
            Encoding::from_string(utf8_singleton_from_raw_parts("encoding", &raw_parts)?)?;

        Ok(Self {
            data: Cow::from(data),
            confidence: Cow::from(confidence),
            label,
            encoding,
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
        let label = utf8_array_from_raw_parts("label", raw_parts)?;

        let data = primitive_array_view_from_raw_parts::<arrow::datatypes::Float32Type>(
            "data", raw_parts,
        )?;

        let confidence = primitive_array_view_from_raw_parts::<arrow::datatypes::Float32Type>(
            "confidence",
            raw_parts,
        )?;

        let encoding =
            Encoding::from_string(utf8_singleton_from_raw_parts("encoding", raw_parts)?)?;

        Ok(Self {
            data: Cow::from(data),
            confidence: Cow::from(confidence),
            label,
            encoding,
        })
    }

    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_parts(Self::raw_parts(array_data)?)
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::array::Array;

        let data: arrow::array::ArrayRef =
            Arc::new(arrow::array::Float32Array::from(self.data.into_owned()));

        let confidence: arrow::array::ArrayRef = Arc::new(arrow::array::Float32Array::from(
            self.confidence.into_owned(),
        ));

        let label: arrow::array::ArrayRef = Arc::new(arrow::array::StringArray::from(self.label));
        let encoding: arrow::array::ArrayRef = Arc::new(arrow::array::StringArray::from(vec![
            self.encoding
                .to_string();
            1
        ]));

        let children = vec![data, confidence, label, encoding];

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

        let union_fields = vec![
            union_field(0, "data", arrow::datatypes::DataType::Float32, false),
            union_field(1, "confidence", arrow::datatypes::DataType::Float32, false),
            union_field(2, "label", arrow::datatypes::DataType::Utf8, false),
            union_field(3, "encoding", arrow::datatypes::DataType::Utf8, false),
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
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = xyxy_bbox.data.as_ptr();

        let arrow_bbox = xyxy_bbox.into_arrow().unwrap();

        let new_bbox = BBox::from_arrow(arrow_bbox).unwrap();
        let final_bbox_buffer = new_bbox.data.as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_eq!(bbox_buffer_address, final_bbox_buffer);
    }
}
