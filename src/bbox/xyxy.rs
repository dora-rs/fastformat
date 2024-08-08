use super::{
    encoding::Encoding, BBox, BboxNdArrayResult, BboxNdArrayViewMutResult, BboxNdArrayViewResult,
};
use eyre::{Context, Report, Result};

impl BBox {
    pub fn new_xyxy(data: Vec<f32>, confidence: Vec<f32>, label: Vec<String>) -> Result<Self> {
        if confidence.len() != label.len() || confidence.len() * 4 != data.len() {
            return Err(Report::msg(
                "Confidence, Label and Data doesn't match length",
            ));
        }

        Ok(BBox {
            data,
            confidence,
            label,
            encoding: Encoding::XYXY,
        })
    }

    pub fn xyxy_from_ndarray(
        data: ndarray::Array<f32, ndarray::Ix1>,
        confidence: ndarray::Array<f32, ndarray::Ix1>,
        label: ndarray::Array<String, ndarray::Ix1>,
    ) -> Result<Self> {
        Self::new_xyxy(
            data.into_raw_vec(),
            confidence.into_raw_vec(),
            label.into_raw_vec(),
        )
    }

    pub fn xyxy_into_ndarray(self) -> Result<BboxNdArrayResult> {
        match self.encoding {
            Encoding::XYXY => Ok((
                ndarray::Array::from_vec(self.data),
                ndarray::Array::from_vec(self.confidence),
                ndarray::Array::from_vec(self.label),
            )),
            _ => Err(Report::msg("BBox is not in XYXY format")),
        }
    }

    pub fn xyxy_into_ndarray_view(&self) -> Result<BboxNdArrayViewResult> {
        match self.encoding {
            Encoding::XYXY => {
                let data = ndarray::ArrayView::from_shape(self.data.len(), &self.data)
                    .wrap_err("Failed to reshape data into ndarray")?;
                let confidence =
                    ndarray::ArrayView::from_shape(self.confidence.len(), &self.confidence)
                        .wrap_err("Failed to reshape data into ndarray")?;
                let label = ndarray::ArrayView::from_shape(self.label.len(), &self.label)
                    .wrap_err("Failed to reshape data into ndarray")?;

                Ok((data, confidence, label))
            }
            _ => Err(Report::msg("BBox is not in XYXY format")),
        }
    }

    pub fn xyxy_into_ndarray_view_mut(&mut self) -> Result<BboxNdArrayViewMutResult> {
        match self.encoding {
            Encoding::XYXY => {
                let data = ndarray::ArrayViewMut::from_shape(self.data.len(), &mut self.data)
                    .wrap_err("Failed to reshape data into ndarray")?;
                let confidence =
                    ndarray::ArrayViewMut::from_shape(self.confidence.len(), &mut self.confidence)
                        .wrap_err("Failed to reshape data into ndarray")?;
                let label = ndarray::ArrayViewMut::from_shape(self.label.len(), &mut self.label)
                    .wrap_err("Failed to reshape data into ndarray")?;

                Ok((data, confidence, label))
            }
            _ => Err(Report::msg("BBox is not in XYXY format")),
        }
    }
}

mod tests {
    #[test]
    fn test_xyxy_creation() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
    }

    #[test]
    fn test_xyxy_from_ndarray() {
        use ndarray::Array1;

        use crate::bbox::BBox;

        let data = Array1::<f32>::zeros(8);
        let confidence = Array1::<f32>::ones(2);
        let label = Array1::<String>::from_vec(vec!["cat".to_string(), "car".to_string()]);

        BBox::xyxy_from_ndarray(data, confidence, label).unwrap();
    }

    #[test]
    fn test_xyxy_into_ndarray() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();

        bbox.xyxy_into_ndarray().unwrap();
    }

    #[test]
    fn test_xyxy_into_ndarray_view() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();

        bbox.xyxy_into_ndarray_view().unwrap();
    }

    #[test]
    fn test_xyxy_into_ndarray_view_mut() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let mut bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();

        bbox.xyxy_into_ndarray_view_mut().unwrap();
    }

    #[test]
    fn test_xyxy_ndarray_zero_copy_conversion() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = bbox.data.as_ptr();

        let (data, confidence, label) = bbox.xyxy_into_ndarray().unwrap();
        let ndarray_buffer_address = data.as_ptr();

        let final_bbox = BBox::xyxy_from_ndarray(data, confidence, label).unwrap();
        let final_bbox_buffer_address = final_bbox.data.as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_eq!(bbox_buffer_address, ndarray_buffer_address);
        assert_eq!(ndarray_buffer_address, final_bbox_buffer_address);
    }
}
