use super::{encoding::Encoding, Image};
use eyre::{Context, Report, Result};

use fastformat_converter::ndarray::{Ndarray, NdarrayView, NdarrayViewMut};

pub type NdarrayImage = (Ndarray, Encoding, Option<String>);
pub type NdarrayImageView<'a> = (NdarrayView<'a>, Encoding, Option<&'a str>);
pub type NdarrayImageViewMut<'a> = (NdarrayViewMut<'a>, Encoding, Option<&'a str>);

impl Image<'_> {
    pub fn from_ndarray(ndarray: NdarrayImage) -> Result<Self> {
        match ndarray {
            (Ndarray::U8IX3(array), Encoding::BGR8, name) => {
                let width = array.shape()[1] as u32;
                let height = array.shape()[0] as u32;

                let (data, _) = array.into_raw_vec_and_offset();

                Self::new_bgr8(data, width, height, name.as_deref())
            }
            (Ndarray::U8IX3(array), Encoding::RGB8, name) => {
                let width = array.shape()[1] as u32;
                let height = array.shape()[0] as u32;

                let (data, _) = array.into_raw_vec_and_offset();

                Self::new_rgb8(data, width, height, name.as_deref())
            }
            (Ndarray::U8IX2(array), Encoding::GRAY8, name) => {
                let width = array.shape()[1] as u32;
                let height = array.shape()[0] as u32;

                let (data, _) = array.into_raw_vec_and_offset();

                Self::new_gray8(data, width, height, name.as_deref())
            }
            _ => Err(Report::msg("Invalid Ndarray type")).context("from_ndarray"),
        }
    }

    pub fn into_ndarray(self) -> Result<NdarrayImage> {
        match self.encoding {
            Encoding::BGR8 => {
                let ndarray = ndarray::Array::from_shape_vec(
                                    (self.height as usize, self.width as usize, 3),
                                    self.data.into_u8()?,
                                )
                                .wrap_err("Failed to reshape data into ndarray: width, height and BGR8 encoding doesn't match data data length.");

                ndarray.map(|array| (Ndarray::U8IX3(array), self.encoding, self.name))
            }
            Encoding::RGB8 => {
                let ndarray = ndarray::Array::from_shape_vec(
                                    (self.height as usize, self.width as usize, 3),
                                    self.data.into_u8()?,
                                )
                                .wrap_err("Failed to reshape data into ndarray: width, height and RGB8 encoding doesn't match data data length.");

                ndarray.map(|array| (Ndarray::U8IX3(array), self.encoding, self.name))
            }
            Encoding::GRAY8 => {
                let ndarray = ndarray::Array::from_shape_vec(
                                    (self.height as usize, self.width as usize),
                                    self.data.into_u8()?,
                                )
                                .wrap_err("Failed to reshape data into ndarray: width, height and GRAY8 encoding doesn't match data data length.");

                ndarray.map(|array| (Ndarray::U8IX2(array), self.encoding, self.name))
            }
        }
    }
}

impl<'a> Image<'a> {
    pub fn to_ndarray_view(&'a self) -> Result<(NdarrayView<'a>, Encoding, Option<&str>)> {
        match self.encoding {
            Encoding::BGR8 => {
                let array = ndarray::ArrayView3::from_shape(
                        (self.height as usize, self.width as usize, 3),
                        self.data.as_u8()?,
                    )
                    .wrap_err("Failed to create ndarray view: width, height and BGR8 encoding doesn't match data data length.");

                array.map(|array| {
                    (
                        NdarrayView::U8IX3(array),
                        self.encoding,
                        self.name.as_deref(),
                    )
                })
            }
            Encoding::RGB8 => {
                let array = ndarray::ArrayView3::from_shape(
                        (self.height as usize, self.width as usize, 3),
                        self.data.as_u8()?,
                    )
                    .wrap_err("Failed to create ndarray view: width, height and RGB8 encoding doesn't match data data length.");

                array.map(|array| {
                    (
                        NdarrayView::U8IX3(array),
                        self.encoding,
                        self.name.as_deref(),
                    )
                })
            }
            Encoding::GRAY8 => {
                let array = ndarray::ArrayView2::from_shape(
                        (self.height as usize, self.width as usize),
                        self.data.as_u8()?,
                    )
                    .wrap_err("Failed to create ndarray view: width, height and GRAY8 encoding doesn't match data data length.");

                array.map(|array| {
                    (
                        NdarrayView::U8IX2(array),
                        self.encoding,
                        self.name.as_deref(),
                    )
                })
            }
        }
    }

    pub fn to_ndarray_view_mut(
        &'a mut self,
    ) -> Result<(NdarrayViewMut<'a>, Encoding, Option<&str>)> {
        match self.encoding {
            Encoding::BGR8 => {
                let array = ndarray::ArrayViewMut3::from_shape(
                        (self.height as usize, self.width as usize, 3),
                        self.data.as_mut_u8()?,
                    )
                    .wrap_err("Failed to create ndarray view: width, height and BGR8 encoding doesn't match data data length.");

                array.map(|array| {
                    (
                        NdarrayViewMut::U8IX3(array),
                        self.encoding,
                        self.name.as_deref(),
                    )
                })
            }
            Encoding::RGB8 => {
                let array = ndarray::ArrayViewMut3::from_shape(
                        (self.height as usize, self.width as usize, 3),
                        self.data.as_mut_u8()?,
                    )
                    .wrap_err("Failed to create ndarray view: width, height and RGB8 encoding doesn't match data data length.");

                array.map(|array| {
                    (
                        NdarrayViewMut::U8IX3(array),
                        self.encoding,
                        self.name.as_deref(),
                    )
                })
            }
            Encoding::GRAY8 => {
                let array = ndarray::ArrayViewMut2::from_shape(
                        (self.height as usize, self.width as usize),
                        self.data.as_mut_u8()?,
                    )
                    .wrap_err("Failed to create ndarray view: width, height and GRAY8 encoding doesn't match data data length.");

                array.map(|array| {
                    (
                        NdarrayViewMut::U8IX2(array),
                        self.encoding,
                        self.name.as_deref(),
                    )
                })
            }
        }
    }
}

mod tests {
    #[test]
    fn test_bgr8_from_ndarray() {
        use crate::image::Image;
        use fastformat_converter::ndarray::Ndarray;

        let array = Ndarray::U8IX3(ndarray::Array3::<u8>::zeros((3, 3, 3)));

        let image = Image::from_ndarray((array, crate::image::Encoding::BGR8, None)).unwrap();

        assert_eq!(image.encoding, crate::image::Encoding::BGR8)
    }

    #[test]
    fn test_bgr8_into_ndarray() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.into_ndarray().unwrap();
    }

    #[test]
    fn test_bgr8_into_ndarray_view() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.to_ndarray_view().unwrap();
    }

    #[test]
    fn test_bgr8_into_ndarray_view_mut() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let mut image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        image.to_ndarray_view_mut().unwrap();
    }

    #[test]
    fn test_bgr8_ndarray_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let bgr8_ndarray = bgr8_image.into_ndarray().unwrap();
        let ndarray_buffer_address = bgr8_ndarray.0.as_ptr();

        let final_image = Image::from_ndarray(bgr8_ndarray).unwrap();
        let final_image_buffer_address = final_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, ndarray_buffer_address);
        assert_eq!(ndarray_buffer_address, final_image_buffer_address);
    }
}
