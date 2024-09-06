use eyre::{ContextCompat, Result};
use pyo3::prelude::*;

use encoding::Encoding;

use std::borrow::Cow;

mod xywh;
mod xyxy;

#[cfg(feature = "arrow")]
mod arrow;

#[cfg(feature = "ndarray")]
mod ndarray;

mod encoding;

pub struct BBox<'a> {
    pub data: Cow<'a, [f32]>,
    pub confidence: Cow<'a, [f32]>,
    pub label: Vec<String>,
    pub encoding: Encoding,
}

#[pyclass]
pub struct PyBBox {
    pub bbox: BBox<'static>,
}

impl BBox<'_> {
    pub fn into_xyxy(self) -> Result<Self> {
        match self.encoding {
            Encoding::XYWH => {
                let mut data = self.data;
                {
                    let data = data.to_mut();

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
                {
                    let data = data.to_mut();

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

#[pymethods]
impl PyBBox {}

#[pymodule]
pub fn bbox(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBBox>()?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
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

        assert_eq!(expected_bbox, final_bbox_data.into_owned());
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

        assert_eq!(expected_bbox, final_bbox_data.into_owned());
    }
}
