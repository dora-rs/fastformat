use crate::arrow::{column_by_name, union_field, union_look_up_table};

use std::{mem, sync::Arc};

use eyre::{Context, Report, Result};

#[derive(Debug)]
pub struct ImageData<T> {
    pixels: Vec<T>,

    width: u32,
    height: u32,

    name: Option<String>,
}

pub enum Image {
    ImageRGB8(ImageData<u8>),
    ImageBGR8(ImageData<u8>),
    ImageGray8(ImageData<u8>),
}

impl Image {
    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Self::ImageRGB8(image) => image.pixels.as_ptr(),
            Self::ImageBGR8(image) => image.pixels.as_ptr(),
            Self::ImageGray8(image) => image.pixels.as_ptr(),
        }
    }

    pub fn new_bgr8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Self {
        Self::ImageBGR8(ImageData {
            pixels,
            width,
            height,
            name: name.map(|s| s.to_string()),
        })
    }

    pub fn new_rgb8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Self {
        Self::ImageRGB8(ImageData {
            pixels,
            width,
            height,
            name: name.map(|s| s.to_string()),
        })
    }

    pub fn new_gray8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Self {
        Self::ImageGray8(ImageData {
            pixels,
            width,
            height,
            name: name.map(|s| s.to_string()),
        })
    }

    pub fn to_rgb(self) -> Result<Self> {
        match self {
            Self::ImageBGR8(image) => {
                let mut pixels = image.pixels;

                for i in (0..pixels.len()).step_by(3) {
                    pixels.swap(i, i + 2);
                }

                Ok(Self::ImageRGB8(ImageData {
                    pixels,
                    width: image.width,
                    height: image.height,
                    name: image.name.clone(),
                }))
            }
            Self::ImageRGB8(_) => Ok(self),
            Self::ImageGray8(_) => Err(Report::msg("Can't convert grayscale image to RGB")),
        }
    }

    pub fn to_bgr(self) -> Result<Self> {
        match self {
            Self::ImageRGB8(image) => {
                let mut pixels = image.pixels;

                for i in (0..pixels.len()).step_by(3) {
                    pixels.swap(i, i + 2);
                }

                Ok(Self::ImageBGR8(ImageData {
                    pixels,
                    width: image.width,
                    height: image.height,
                    name: image.name.clone(),
                }))
            }
            Self::ImageBGR8(_) => Ok(self),
            Self::ImageGray8(_) => Err(Report::msg("Can't convert grayscale image to BGR")),
        }
    }

    pub fn from_rgb8_ndarray(array: ndarray::Array<u8, ndarray::Ix3>, name: Option<&str>) -> Self {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let pixels = array.into_raw_vec();

        Self::new_rgb8(pixels, width, height, name)
    }

    pub fn from_bgr8_ndarray(array: ndarray::Array<u8, ndarray::Ix3>, name: Option<&str>) -> Self {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let pixels = array.into_raw_vec();

        Self::new_bgr8(pixels, width, height, name)
    }

    pub fn from_gray8_ndarray(array: ndarray::Array<u8, ndarray::Ix2>, name: Option<&str>) -> Self {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let pixels = array.into_raw_vec();

        Self::new_gray8(pixels, width, height, name)
    }

    pub fn to_rgb8_ndarray(self) -> Result<ndarray::Array<u8, ndarray::Ix3>> {
        match self {
            Self::ImageRGB8(image) => ndarray::Array::from_shape_vec(
                (image.height as usize, image.width as usize, 3),
                image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and RGB8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in RGB8 format")),
        }
    }

    pub fn to_bgr8_ndarray(self) -> Result<ndarray::Array<u8, ndarray::Ix3>> {
        match self {
            Self::ImageBGR8(image) => ndarray::Array::from_shape_vec(
                (image.height as usize, image.width as usize, 3),
                image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and BGR8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in BGR8 format")),
        }
    }

    pub fn to_gray8_ndarray(self) -> Result<ndarray::Array<u8, ndarray::Ix2>> {
        match self {
            Self::ImageGray8(image) => ndarray::Array::from_shape_vec(
                (image.height as usize, image.width as usize),
                image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and Gray8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in Gray8 format")),
        }
    }

    pub fn to_rgb8_ndarray_view(&self) -> Result<ndarray::ArrayView<u8, ndarray::Ix3>> {
        match self {
            Self::ImageRGB8(image) => ndarray::ArrayView::from_shape(
                (image.height as usize, image.width as usize, 3),
                &image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and RGB8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in RGB8 format")),
        }
    }

    pub fn to_bgr8_ndarray_view(&self) -> Result<ndarray::ArrayView<u8, ndarray::Ix3>> {
        match self {
            Self::ImageBGR8(image) => ndarray::ArrayView::from_shape(
                (image.height as usize, image.width as usize, 3),
                &image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and BGR8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in BGR8 format")),
        }
    }

    pub fn to_gray8_ndarray_view(&self) -> Result<ndarray::ArrayView<u8, ndarray::Ix2>> {
        match self {
            Self::ImageGray8(image) => ndarray::ArrayView::from_shape(
                (image.height as usize, image.width as usize),
                &image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and Gray8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in Gray8 format")),
        }
    }

    pub fn to_rgb8_ndarray_view_mut(&mut self) -> Result<ndarray::ArrayViewMut<u8, ndarray::Ix3>> {
        match self {
            Self::ImageRGB8(image) => ndarray::ArrayViewMut::from_shape(
                (image.height as usize, image.width as usize, 3),
                &mut image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and RGB8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in RGB8 format")),
        }
    }

    pub fn to_bgr8_ndarray_view_mut(&mut self) -> Result<ndarray::ArrayViewMut<u8, ndarray::Ix3>> {
        match self {
            Self::ImageBGR8(image) => ndarray::ArrayViewMut::from_shape(
                (image.height as usize, image.width as usize, 3),
                &mut image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and BGR8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in BGR8 format")),
        }
    }

    pub fn to_gray8_ndarray_view_mut(&mut self) -> Result<ndarray::ArrayViewMut<u8, ndarray::Ix2>> {
        match self {
            Self::ImageGray8(image) => ndarray::ArrayViewMut::from_shape(
                (image.height as usize, image.width as usize),
                &mut image.pixels,
            )
            .wrap_err("Failed to reshape pixels into ndarray: width, height and Gray8 encoding doesn't match pixels data length."),
            _ => Err(Report::msg("Image is not in Gray8 format")),
        }
    }

    pub fn from_arrow(array: arrow::array::UnionArray) -> Result<Self> {
        use arrow::array::Array;

        let union_fields = match array.data_type() {
            arrow::datatypes::DataType::Union(fields, ..) => fields,
            _ => {
                return Err(Report::msg("UnionArray has invalid data type."));
            }
        };

        let look_up_table = union_look_up_table(&union_fields);

        let width =
            column_by_name::<arrow::array::UInt32Array>(&array, "width", &look_up_table)?.value(0);
        let height =
            column_by_name::<arrow::array::UInt32Array>(&array, "height", &look_up_table)?.value(0);
        let encoding =
            column_by_name::<arrow::array::StringArray>(&array, "encoding", &look_up_table)?
                .value(0)
                .to_string();

        let name = column_by_name::<arrow::array::StringArray>(&array, "name", &look_up_table)?;

        let name = if name.is_null(0) {
            None
        } else {
            Some(name.value(0).to_string())
        };

        let name = name.as_ref().map(|s| s.as_str());

        unsafe {
            let array = mem::ManuallyDrop::new(array);
            let pixels = match encoding.as_str() {
                "RGB8" => {
                    column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table)?
                }
                "BGR8" => {
                    column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table)?
                }
                "GRAY8" => {
                    column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table)?
                }
                _ => {
                    return Err(Report::msg(format!("Invalid encoding: {}", encoding)));
                }
            };

            let ptr = pixels.values().as_ptr();
            let len = pixels.len();

            let pixels = Vec::from_raw_parts(ptr as *mut u8, len, len);

            return match encoding.as_str() {
                "RGB8" => Ok(Self::new_rgb8(pixels, width, height, name)),
                "BGR8" => Ok(Self::new_bgr8(pixels, width, height, name)),
                "GRAY8" => Ok(Self::new_gray8(pixels, width, height, name)),
                _ => Err(Report::msg(format!("Invalid encoding: {}", encoding))),
            };
        }
    }

    fn get_image_details<T>(
        image: &ImageData<T>,
    ) -> (
        arrow::array::UInt32Array,
        arrow::array::UInt32Array,
        arrow::array::StringArray,
        arrow::array::StringArray,
    ) {
        let width = arrow::array::UInt32Array::from(vec![image.width; 1]);
        let height = arrow::array::UInt32Array::from(vec![image.height; 1]);
        let encoding = arrow::array::StringArray::from(vec!["BGR8".to_string(); 1]);

        let name = arrow::array::StringArray::from(vec![image.name.clone(); 1]);

        (width, height, encoding, name)
    }

    pub fn to_arrow(self) -> Result<arrow::array::UnionArray> {
        let ((width, height, encoding, name), pixels, datatype) = match self {
            Image::ImageBGR8(image) => (
                Self::get_image_details(&image),
                arrow::array::UInt8Array::from(image.pixels),
                arrow::datatypes::DataType::UInt8,
            ),
            Image::ImageRGB8(image) => (
                Self::get_image_details(&image),
                arrow::array::UInt8Array::from(image.pixels),
                arrow::datatypes::DataType::UInt8,
            ),
            Image::ImageGray8(image) => (
                Self::get_image_details(&image),
                arrow::array::UInt8Array::from(image.pixels),
                arrow::datatypes::DataType::UInt8,
            ),
        };

        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

        let union_fields = [
            union_field(0, "pixels", datatype, false),
            union_field(1, "width", arrow::datatypes::DataType::UInt32, false),
            union_field(2, "height", arrow::datatypes::DataType::UInt32, false),
            union_field(3, "encoding", arrow::datatypes::DataType::Utf8, false),
            union_field(4, "name", arrow::datatypes::DataType::Utf8, true),
        ]
        .into_iter()
        .collect::<arrow::datatypes::UnionFields>();

        let children: Vec<Arc<dyn arrow::array::Array>> = vec![
            Arc::new(pixels),
            Arc::new(width),
            Arc::new(height),
            Arc::new(encoding),
            Arc::new(name),
        ];

        arrow::array::UnionArray::try_new(union_fields, type_ids, Some(offsets), children)
            .wrap_err("Failed to create UnionArray")
    }
}
