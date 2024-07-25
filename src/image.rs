use std::cmp::PartialEq;
use std::mem;
use std::sync::Arc;
use arrow::array::{StringArray, UInt32Array, UInt8Array, UnionArray};
use arrow::buffer::ScalarBuffer;
use arrow::datatypes::{DataType, Field, UnionFields};
use arrow::error::ArrowError;
use ndarray::{Array, Ix3};
use crate::image::Encoding::{BGR8, RGB8};

#[derive(PartialEq)]
pub enum Encoding {
    BGR8,
    RGB8,
    Unknown,
}

impl Encoding {
    pub fn to_string(&self) -> String {
        match self {
            BGR8 => "bgr8".to_string(),
            RGB8 => "rgb8".to_string(),
            Encoding::Unknown => "unknown".to_string(),
        }
    }

    pub fn from_string(encoding: &str) -> Self {
        match encoding {
            "bgr8" => BGR8,
            "rgb8" => RGB8,
            _ => Encoding::Unknown,
        }
    }
}

pub struct Image<T> {
    width: u32,
    height: u32,
    base_encoding: Encoding,
    data: Vec<T>,
}

impl<T> Image<T> {
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn from_flat(data: Vec<T>, width: u32, height: u32, base_encoding: Encoding) -> Self {
        Image {
            width,
            height,
            base_encoding,
            data,
        }
    }

    pub fn from_nd_array(nd_array: Array<T, Ix3>, base_encoding: Encoding) -> Self
    {
        let shape = nd_array.shape();

        let width = shape[1] as u32;
        let height = shape[0] as u32;
        let channels = shape[2] as u32;

        let flat_size = (width * height * channels) as usize;

        let reshaped_nd_array = nd_array.into_shape(flat_size).unwrap();
        let data = reshaped_nd_array.into_raw_vec();

        Image {
            width,
            height,
            base_encoding,
            data,
        }
    }

    pub fn to_flat(self) -> Vec<T> {
        self.data
    }

    pub fn to_nd_array(self, encoding: Encoding) -> Result<Array<T, Ix3>, ()>
    {
        if (self.base_encoding == BGR8 && encoding == RGB8) || (self.base_encoding == RGB8 && encoding == BGR8) {
            let mut array: Array<T, Ix3> = Array::from_shape_vec((self.height as usize, self.width as usize, 3), self.data).unwrap();
            array.swap_axes(2, 0);

            return Ok(array);
        }

        if self.base_encoding == encoding && (self.base_encoding == BGR8 || self.base_encoding == RGB8) {
            let array: Array<T, Ix3> = Array::from_shape_vec((self.height as usize, self.width as usize, 3), self.data).unwrap();
            return Ok(array);
        }

        Err(())
    }
}

impl Image<u8> {
    pub fn from_arrow_array(arrow_array: UnionArray) -> Self
    {
        unsafe {
            let mut data = mem::ManuallyDrop::new(arrow_array);
            let shape = data.child(0).as_any().downcast_ref::<UInt32Array>().unwrap();
            let width = shape.value(0);
            let height = shape.value(1);

            let base_encoding = data.child(1).as_any().downcast_ref::<StringArray>().unwrap().value(0);

            if base_encoding != "bgr8" && base_encoding != "rgb8" {
                return Image {
                    width,
                    height,
                    base_encoding: Encoding::Unknown,
                    data: vec![],
                };
            }

            let ptr = data.child(2).as_any().downcast_ref::<UInt8Array>().unwrap().values().as_ptr();
            let len = data.child(2).as_any().downcast_ref::<UInt8Array>().unwrap().len();
            let data = Vec::from_raw_parts(ptr as *mut u8, len, len);

            return Image {
                width,
                height,
                base_encoding: Encoding::from_string(base_encoding),
                data,
            };
        }
    }

    pub fn to_arrow_array(self) -> Result<UnionArray, ArrowError> {
        let shape = UInt32Array::from([self.width, self.height].to_vec());
        let encoding = StringArray::from([self.base_encoding.to_string()].to_vec());
        let data = UInt8Array::from(self.data);

        let type_ids = [].into_iter().collect::<ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<ScalarBuffer<i32>>();

        let union_fields = [
            (0, Arc::new(Field::new("Shape", DataType::UInt32, false))),
            (1, Arc::new(Field::new("Encoding", DataType::Utf8, false))),
            (2, Arc::new(Field::new("Data", DataType::UInt8, false))),
        ].into_iter().collect::<UnionFields>();

        let children: Vec<Arc<dyn arrow::array::Array>> = vec![
            Arc::new(shape),
            Arc::new(encoding),
            Arc::new(data),
        ];

        UnionArray::try_new(
            union_fields,
            type_ids,
            Some(offsets),
            children,
        )
    }
}