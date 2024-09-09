use std::borrow::Cow;

use super::{encoding::Encoding, BBox};
use eyre::{Report, Result};

impl BBox<'_> {
    pub fn new_xywh(data: Vec<f32>, confidence: Vec<f32>, label: Vec<String>) -> Result<Self> {
        if confidence.len() != label.len()
            || confidence.len() * 4 != data.len()
            || label.len() * 4 != data.len()
        {
            return Err(Report::msg(
                "Confidence, Label and Data doesn't match length",
            ));
        }

        Ok(BBox {
            data: Cow::from(data),
            confidence: Cow::from(confidence),
            label,
            encoding: Encoding::XYWH,
        })
    }
}

mod tests {
    #[test]
    fn test_xywh_creation() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 1.0, 1.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        BBox::new_xywh(flat_bbox, confidence, label).unwrap();
    }
}
