use crate::arrow::{column_by_name, union_field, union_lookup_table};

use super::{encoding::Encoding, BBox};
use eyre::{Context, Report, Result};

use std::{collections::HashMap, mem, sync::Arc};

impl BBox {
    unsafe fn arrow_to_vec<T: arrow::datatypes::ArrowPrimitiveType, G>(
        field: &str,
        array: &arrow::array::UnionArray,
        lookup_table: &HashMap<String, i8>,
    ) -> Result<Vec<G>> {
        let arrow = column_by_name::<arrow::array::PrimitiveArray<T>>(array, field, lookup_table)?;
        let ptr = arrow.values().as_ptr();
        let len = arrow.len();

        Ok(Vec::from_raw_parts(ptr as *mut G, len, len))
    }

    fn convert_bbox_details_into_arrow(bbox: BBox) -> Result<Vec<Arc<dyn arrow::array::Array>>> {
        let data = Arc::new(arrow::array::Float32Array::from(bbox.data));
        let confidence = Arc::new(arrow::array::Float32Array::from(bbox.confidence));
        let label = Arc::new(arrow::array::StringArray::from(bbox.label));
        let encoding = Arc::new(arrow::array::StringArray::from(vec![
            bbox.encoding
                .to_string();
            1
        ]));

        Ok(vec![data, confidence, label, encoding])
    }

    pub fn into_arrow(self) -> Result<arrow::array::UnionArray> {
        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

        let union_fields = [
            union_field(0, "data", arrow::datatypes::DataType::Float32, false),
            union_field(1, "confidence", arrow::datatypes::DataType::Float32, false),
            union_field(2, "label", arrow::datatypes::DataType::Utf8, false),
            union_field(3, "encoding", arrow::datatypes::DataType::Utf8, false),
        ]
        .into_iter()
        .collect::<arrow::datatypes::UnionFields>();

        let children = Self::convert_bbox_details_into_arrow(self)?;

        arrow::array::UnionArray::try_new(union_fields, type_ids, Some(offsets), children)
            .wrap_err("Failed to create UnionArray with BBox data.")
    }

    pub fn from_arrow(array: arrow::array::UnionArray) -> Result<Self> {
        use arrow::array::Array;

        let union_fields = match array.data_type() {
            arrow::datatypes::DataType::Union(fields, ..) => fields,
            _ => {
                return Err(Report::msg("UnionArray has invalid data type."));
            }
        };

        let lookup_table = union_lookup_table(union_fields);

        let encoding = Encoding::from_string(
            column_by_name::<arrow::array::StringArray>(&array, "encoding", &lookup_table)?
                .value(0)
                .to_string(),
        )?;

        unsafe {
            let array = mem::ManuallyDrop::new(array);

            let data = Self::arrow_to_vec::<arrow::datatypes::Float32Type, f32>(
                "data",
                &array,
                &lookup_table,
            )?;

            let confidence = Self::arrow_to_vec::<arrow::datatypes::Float32Type, f32>(
                "confidence",
                &array,
                &lookup_table,
            )?;

            let arrow =
                column_by_name::<arrow::array::StringArray>(&array, "label", &lookup_table)?;
            let ptr = arrow.values().as_ptr();
            let len = arrow.len();

            let label = Vec::from_raw_parts(ptr as *mut String, len, len);

            Ok(BBox {
                data,
                confidence,
                label,
                encoding,
            })
        }
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
