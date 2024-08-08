use eyre::{ContextCompat, Result};

use encoding::Encoding;

mod xywh;
mod xyxy;

mod arrow;

mod encoding;

pub struct BBox {
    pub data: Vec<f32>,
    pub confidence: Vec<f32>,
    pub label: Vec<String>,
    pub encoding: Encoding,
}

impl BBox {
    pub fn into_xyxy(self) -> Result<Self> {
        match self.encoding {
            Encoding::XYWH => {
                let mut data = self.data;

                for i in 0..self.confidence.len() {
                    let x = data
                        .get(i * 4)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;
                    let y = data
                        .get(i * 4 + 1)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;
                    let w = data
                        .get(i * 4 + 2)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;
                    let h = data
                        .get(i * 4 + 3)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;

                    data[i * 4 + 2] = x + w;
                    data[i * 4 + 3] = y + h;
                }

                Ok(Self {
                    data,
                    confidence: self.confidence,
                    label: self.label,
                    encoding: self.encoding,
                })
            }
            Encoding::XYXY => Ok(self),
        }
    }

    pub fn into_xywh(self) -> Result<Self> {
        match self.encoding {
            Encoding::XYXY => {
                let mut data = self.data;

                for i in 0..self.confidence.len() {
                    let x1 = data
                        .get(i * 4)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;
                    let y1 = data
                        .get(i * 4 + 1)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;
                    let x2 = data
                        .get(i * 4 + 2)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;
                    let y2 = data
                        .get(i * 4 + 3)
                        .wrap_err("Not enough data matching 4 values per box!")
                        .cloned()?;

                    data[i * 4 + 2] = x2 - x1;
                    data[i * 4 + 3] = y2 - y1;
                }

                Ok(Self {
                    data,
                    confidence: self.confidence,
                    label: self.label,
                    encoding: self.encoding,
                })
            }
            Encoding::XYWH => Ok(self),
        }
    }
}

mod tests {
    #[test]
    fn test_xyxy_into_xywh() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let final_bbox = bbox.into_xywh().unwrap();
        let final_bbox_data = final_bbox.data;

        let expected_bbox = vec![1.0, 1.0, 1.0, 1.0];

        assert_eq!(expected_bbox, final_bbox_data);
    }

    #[test]
    fn test_xywh_into_xyxy() {
        use crate::bbox::BBox;

        let flat_bbox = vec![1.0, 1.0, 1.0, 1.0];
        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let bbox = BBox::new_xywh(flat_bbox, confidence, label).unwrap();
        let final_bbox = bbox.into_xyxy().unwrap();
        let final_bbox_data = final_bbox.data;

        let expected_bbox = vec![1.0, 1.0, 2.0, 2.0];

        assert_eq!(expected_bbox, final_bbox_data);
    }
}
