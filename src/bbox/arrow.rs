use crate::arrow::{arrow_union_into_map, get_primitive_array_from_map, get_utf8_array_from_map};

use super::{encoding::Encoding, BBox};
use eyre::{Context, ContextCompat, Report, Result};

use std::sync::Arc;

impl BBox {
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

    /// Converts a `BBox` instance into an Arrow `UnionArray`.
    ///
    /// This function takes a `BBox` and converts it into an Arrow `UnionArray`, which contains
    /// the fields `data`, `confidence`, `label`, and `encoding` as different children arrays
    /// within the union.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `arrow::array::UnionArray` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if the `UnionArray` construction fails due to any issue with the `BBox` fields
    /// or the conversion process.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::bbox::BBox;
    ///
    /// let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
    /// let confidence = vec![0.98];
    /// let label = vec!["cat".to_string()];
    /// let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
    ///
    /// let arrow_union = xyxy_bbox.into_arrow().unwrap();
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

    /// Creates a `BBox` instance from an Arrow `UnionArray`.
    ///
    /// This function takes an Arrow `UnionArray`, extracts the `BBox` fields (`data`, `confidence`,
    /// `label`, and `encoding`), and uses them to create a new `BBox` instance.
    ///
    /// # Arguments
    ///
    /// * `array` - An `arrow::array::UnionArray` containing the `BBox` data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `BBox` if successful, or an error otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if any field is missing or if there is an issue during the conversion.
    ///
    /// # Example
    ///
    /// ```
    /// use fastformat::bbox::BBox;
    ///
    /// let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
    /// let confidence = vec![0.98];
    /// let label = vec!["cat".to_string()];
    /// let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
    ///
    /// let arrow_union = xyxy_bbox.into_arrow().unwrap();
    ///
    /// let new_bbox = BBox::from_arrow(arrow_union).unwrap();
    /// ```
    pub fn from_arrow(array: arrow::array::UnionArray) -> Result<Self> {
        let mut map = arrow_union_into_map(array)?;

        let data =
            get_primitive_array_from_map::<f32, arrow::datatypes::Float32Type>("data", &mut map)?;
        let confidence = get_primitive_array_from_map::<f32, arrow::datatypes::Float32Type>(
            "confidence",
            &mut map,
        )?;
        let label = get_utf8_array_from_map("label", &mut map)?;
        let encoding = Encoding::from_string(
            get_utf8_array_from_map("encoding", &mut map)?
                .first()
                .cloned()
                .wrap_err(Report::msg(
                    "encoding field must contains at least 1 value!",
                ))?,
        )?;

        Ok(BBox {
            data,
            confidence,
            label,
            encoding,
        })
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
