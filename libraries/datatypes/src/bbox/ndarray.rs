use super::{encoding::Encoding, BBox};
use eyre::{Context, Report, Result};

use fastformat_converter::ndarray::{Ndarray, NdarrayView, NdarrayViewMut};

pub type NdarrayBBox = (Ndarray, Ndarray, Ndarray, Encoding);
pub type NdarrayBBoxView<'a> = (NdarrayView<'a>, NdarrayView<'a>, NdarrayView<'a>, Encoding);
pub type NdarrayBBoxViewMut<'a> = (
    NdarrayViewMut<'a>,
    NdarrayViewMut<'a>,
    NdarrayViewMut<'a>,
    Encoding,
);

impl BBox<'_> {
    pub fn from_ndarray(ndarray: NdarrayBBox) -> Result<Self> {
        match ndarray {
            (
                Ndarray::F32IX1(data),
                Ndarray::F32IX1(confidence),
                Ndarray::STRIX1(label),
                Encoding::XYXY,
            ) => Self::new_xyxy(
                data.into_raw_vec_and_offset().0,
                confidence.into_raw_vec_and_offset().0,
                label.into_raw_vec_and_offset().0,
            ),
            (
                Ndarray::F32IX1(data),
                Ndarray::F32IX1(confidence),
                Ndarray::STRIX1(label),
                Encoding::XYWH,
            ) => Self::new_xywh(
                data.into_raw_vec_and_offset().0,
                confidence.into_raw_vec_and_offset().0,
                label.into_raw_vec_and_offset().0,
            ),
            _ => Err(Report::msg("Invalid Ndarray type")).context("from_ndarray"),
        }
    }

    pub fn into_ndarray(self) -> Result<NdarrayBBox> {
        Ok((
            Ndarray::F32IX1(
                ndarray::Array::from_shape_vec(self.data.len(), self.data.into_owned())
                    .wrap_err("Failed to reshape data into ndarray")?,
            ),
            Ndarray::F32IX1(
                ndarray::Array::from_shape_vec(self.confidence.len(), self.confidence.into_owned())
                    .wrap_err("Failed to reshape confidence into ndarray")?,
            ),
            Ndarray::STRIX1(
                ndarray::Array::from_shape_vec(self.label.len(), self.label)
                    .wrap_err("Failed to reshape label into ndarray")?,
            ),
            self.encoding,
        ))
    }
}

impl<'a> BBox<'a> {
    pub fn to_ndarray_view(&'a self) -> Result<NdarrayBBoxView<'a>> {
        Ok((
            NdarrayView::F32IX1(
                ndarray::ArrayView::from_shape(self.data.len(), &self.data)
                    .wrap_err("Failed to reshape data into ndarray")?,
            ),
            NdarrayView::F32IX1(
                ndarray::ArrayView::from_shape(self.confidence.len(), &self.confidence)
                    .wrap_err("Failed to reshape confidence into ndarray")?,
            ),
            NdarrayView::STRIX1(
                ndarray::ArrayView::from_shape(self.label.len(), &self.label)
                    .wrap_err("Failed to reshape label into ndarray")?,
            ),
            self.encoding,
        ))
    }

    pub fn to_ndarray_view_mut(&'a mut self) -> Result<NdarrayBBoxViewMut<'a>> {
        Ok((
            NdarrayViewMut::F32IX1(
                ndarray::ArrayViewMut::from_shape(self.data.len(), self.data.to_mut())
                    .wrap_err("Failed to reshape data into ndarray")?,
            ),
            NdarrayViewMut::F32IX1(
                ndarray::ArrayViewMut::from_shape(self.confidence.len(), self.confidence.to_mut())
                    .wrap_err("Failed to reshape confidence into ndarray")?,
            ),
            NdarrayViewMut::STRIX1(
                ndarray::ArrayViewMut::from_shape(self.label.len(), self.label.as_mut())
                    .wrap_err("Failed to reshape label into ndarray")?,
            ),
            self.encoding,
        ))
    }
}
