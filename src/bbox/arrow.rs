use std::borrow::Cow;

use super::{encoding::Encoding, BBox};
use crate::arrow::{RawData, UnionBuilder};

use eyre::Result;

impl<'a> BBox<'a> {
    pub fn raw_data(array_data: arrow::array::ArrayData) -> Result<RawData> {
        use arrow::datatypes::Float32Type;

        let raw_data = RawData::new(array_data)?
            .load_primitive::<Float32Type>("data")?
            .load_primitive::<Float32Type>("confidence")?
            .load_utf("label")?
            .load_utf("encoding")?;

        Ok(raw_data)
    }

    pub fn from_raw_data(mut raw_data: RawData) -> Result<Self> {
        use arrow::datatypes::Float32Type;

        let data = raw_data.primitive_array::<Float32Type>("data")?;
        let confidence = raw_data.primitive_array::<Float32Type>("confidence")?;
        let label = raw_data.utf8_array("label")?;
        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;

        Ok(Self {
            data: Cow::Owned(data),
            confidence: Cow::Owned(confidence),
            label,
            encoding,
        })
    }

    pub fn view_from_raw_data(raw_data: &'a RawData) -> Result<Self> {
        use arrow::datatypes::Float32Type;

        let data = raw_data.primitive_array_view::<Float32Type>("data")?;
        let confidence = raw_data.primitive_array_view::<Float32Type>("confidence")?;
        let label = raw_data.utf8_array("label")?;
        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;

        Ok(Self {
            data: Cow::Borrowed(data),
            confidence: Cow::Borrowed(confidence),
            label,
            encoding,
        })
    }

    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_data(Self::raw_data(array_data)?)
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::datatypes::{
            DataType::{Float32, Utf8},
            Float32Type,
        };

        let raw_data = UnionBuilder::new()
            .push_primitive_array::<Float32Type>("data", self.data.into_owned(), Float32, false)
            .push_primitive_array::<Float32Type>(
                "confidence",
                self.confidence.into_owned(),
                Float32,
                false,
            )
            .push_utf_array("label", self.label, Utf8, false)
            .push_utf_singleton("encoding", self.encoding.to_string(), Utf8, false);

        raw_data.into_arrow()
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

        let xyxy_bbox = BBox::from_arrow(arrow_bbox).unwrap();
        let xyxy_bbox_buffer = xyxy_bbox.data.as_ptr();

        let xywh_bbox = xyxy_bbox.into_xywh().unwrap();
        let xywh_bbox_buffer = xywh_bbox.data.as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_eq!(bbox_buffer_address, xyxy_bbox_buffer);
        assert_eq!(xyxy_bbox_buffer, xywh_bbox_buffer);
    }
}
