use std::borrow::Cow;

use super::{encoding::Encoding, BBox};

use fastformat_converter::arrow::{
    builder::ArrowDataBuilder, consumer::ArrowDataConsumer, viewer::ArrowDataViewer, IntoArrow,
    ViewArrow,
};

impl IntoArrow for BBox<'_> {
    fn into_arrow(self) -> eyre::Result<arrow::array::ArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_array::<arrow::datatypes::Float32Type>("data", self.data.into_owned())
            .push_primitive_array::<arrow::datatypes::Float32Type>(
                "confidence",
                self.confidence.into_owned(),
            )
            .push_utf8_array("label", self.label)
            .push_utf8_singleton("encoding", self.encoding.to_string());

        builder.build()
    }

    fn from_arrow(array_data: arrow::array::ArrayData) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let data = consumer.primitive_array::<arrow::datatypes::Float32Type>("data")?;
        let confidence = consumer.primitive_array::<arrow::datatypes::Float32Type>("confidence")?;
        let label = consumer.utf8_array("label")?;

        let encoding = Encoding::from_string(consumer.utf8_singleton("encoding")?)?;

        Ok(Self {
            data: Cow::Owned(data),
            confidence: Cow::Owned(confidence),
            label,
            encoding,
        })
    }
}

impl<'a> ViewArrow<'a> for BBox<'a> {
    fn viewer(
        array_data: arrow::array::ArrayData,
    ) -> eyre::Result<fastformat_converter::arrow::viewer::ArrowDataViewer> {
        ArrowDataViewer::new(array_data)?
            .load_primitive::<arrow::datatypes::Float32Type>("data")?
            .load_primitive::<arrow::datatypes::Float32Type>("confidence")?
            .load_utf8("label")
    }

    fn view_arrow(viewer: &'a ArrowDataViewer) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let data = viewer.primitive_array::<arrow::datatypes::Float32Type>("data")?;
        let confidence = viewer.primitive_array::<arrow::datatypes::Float32Type>("confidence")?;
        let label = viewer.utf8_array("label")?;

        let encoding = Encoding::from_string(viewer.utf8_singleton("encoding")?)?;

        Ok(Self {
            data: Cow::Borrowed(data),
            confidence: Cow::Borrowed(confidence),
            label,
            encoding,
        })
    }
}

mod tests {
    #[test]
    fn test_arrow_zero_copy_conversion() {
        use crate::bbox::BBox;
        use fastformat_converter::arrow::IntoArrow;

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

    #[test]
    fn test_arrow_zero_copy_read_only() {
        use crate::bbox::BBox;
        use fastformat_converter::arrow::{IntoArrow, ViewArrow};

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = xyxy_bbox.data.as_ptr();

        let arrow_bbox = xyxy_bbox.into_arrow().unwrap();

        let raw_data = BBox::viewer(arrow_bbox).unwrap();
        let xyxy_bbox = BBox::view_arrow(&raw_data).unwrap();
        let xyxy_bbox_buffer = xyxy_bbox.data.as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_eq!(bbox_buffer_address, xyxy_bbox_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_copy_on_write() {
        use crate::bbox::BBox;
        use fastformat_converter::arrow::{IntoArrow, ViewArrow};

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = xyxy_bbox.data.as_ptr();

        let arrow_bbox = xyxy_bbox.into_arrow().unwrap();

        let raw_data = BBox::viewer(arrow_bbox).unwrap();
        let xyxy_bbox = BBox::view_arrow(&raw_data).unwrap();
        let xywh_bbox = xyxy_bbox.into_xywh().unwrap();

        let final_bbox_buffer = xywh_bbox.data.as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_ne!(bbox_buffer_address, final_bbox_buffer);
    }
}
