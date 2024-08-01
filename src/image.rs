use crate::arrow::{column_by_name, union_field, union_look_up_table};

use std::{mem, sync::Arc};

#[derive(PartialEq, Debug)]
enum Encoding {
    BGR8,
    RGB8,
    Unknown,
}

impl Encoding {
    fn to_string(&self) -> String {
        match self {
            Self::BGR8 => "BGR8".to_string(),
            Self::RGB8 => "RGB8".to_string(),
            Self::Unknown => "unknown".to_string(),
        }
    }

    fn from_string(encoding: &str) -> Self {
        match encoding {
            "BGR8" => Self::BGR8,
            "RGB8" => Self::RGB8,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Image<T> {
    pixels: Vec<T>,

    width: u32,
    height: u32,
    encoding: Encoding,

    name: Option<String>,
}

impl<T> Image<T> {
    pub fn as_ptr(&self) -> *const u8 {
        self.pixels.as_ptr() as *const u8
    }
}

impl Image<u8> {
    pub fn new_rgb8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Self {
        Self {
            pixels,
            width,
            height,
            encoding: Encoding::RGB8,
            name: name.map(|s| s.to_string()),
        }
    }

    pub fn new_bgr8(pixels: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> Self {
        Self {
            pixels,
            width,
            height,
            encoding: Encoding::BGR8,
            name: name.map(|s| s.to_string()),
        }
    }

    pub fn to_bgr(self) -> Self {
        if self.encoding == Encoding::BGR8 {
            return self;
        }

        let mut pixels = self.pixels;

        for i in (0..pixels.len()).step_by(3) {
            pixels.swap(i, i + 2);
        }

        Self {
            pixels,
            width: self.width,
            height: self.height,
            encoding: Encoding::BGR8,
            name: self.name,
        }
    }

    pub fn to_rgb(self) -> Self {
        if self.encoding == Encoding::RGB8 {
            return self;
        }

        let mut pixels = self.pixels;

        for i in (0..pixels.len()).step_by(3) {
            pixels.swap(i, i + 2);
        }

        Self {
            pixels,
            width: self.width,
            height: self.height,
            encoding: Encoding::RGB8,
            name: self.name,
        }
    }
}

impl Image<u8> {
    pub fn from_rgb8_nd_array(array: ndarray::Array<u8, ndarray::Ix3>, name: Option<&str>) -> Self {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let flat_size = (width * height * 3) as usize;

        let reshaped_nd_array = array.into_shape(flat_size).unwrap();
        let pixels = reshaped_nd_array.into_raw_vec();

        Self::new_rgb8(pixels, width, height, name)
    }

    pub fn from_bgr8_nd_array(array: ndarray::Array<u8, ndarray::Ix3>, name: Option<&str>) -> Self {
        let width = array.shape()[1] as u32;
        let height = array.shape()[0] as u32;

        let flat_size = (width * height * 3) as usize;

        let reshaped_nd_array = array.into_shape(flat_size).unwrap();
        let pixels = reshaped_nd_array.into_raw_vec();

        Self::new_bgr8(pixels, width, height, name)
    }

    pub fn to_nd_array(self) -> ndarray::Array<u8, ndarray::Ix3> {
        let reshaped_nd_array: ndarray::Array<u8, ndarray::Ix3> = ndarray::Array::from_shape_vec(
            (self.height as usize, self.width as usize, 3),
            self.pixels,
        )
        .unwrap();

        reshaped_nd_array
    }

    pub fn nd_array_view(&self) -> ndarray::ArrayView3<u8> {
        let reshaped_nd_array: ndarray::ArrayView3<u8> = ndarray::ArrayView3::from_shape(
            (self.height as usize, self.width as usize, 3),
            &self.pixels,
        )
        .unwrap();

        reshaped_nd_array
    }
}

impl Image<u8> {
    pub fn from_arrow(array: arrow::array::UnionArray) -> Self {
        use arrow::array::Array;

        let union_fields = match array.data_type() {
            arrow::datatypes::DataType::Union(fields, ..) => fields,
            _ => panic!("Expected data_type to be arrow::datatypes::DataType::Union"),
        };

        let look_up_table = union_look_up_table(&union_fields);

        let width =
            column_by_name::<arrow::array::UInt32Array>(&array, "width", &look_up_table).value(0);
        let height =
            column_by_name::<arrow::array::UInt32Array>(&array, "height", &look_up_table).value(0);
        let encoding = Encoding::from_string(
            &column_by_name::<arrow::array::StringArray>(&array, "encoding", &look_up_table)
                .value(0),
        );

        let name = column_by_name::<arrow::array::StringArray>(&array, "name", &look_up_table);

        let name = if name.is_null(0) {
            None
        } else {
            Some(name.value(0).to_string())
        };

        let name = name.as_ref().map(|s| s.as_str());

        unsafe {
            let array = mem::ManuallyDrop::new(array);
            let pixels =
                column_by_name::<arrow::array::UInt8Array>(&array, "pixels", &look_up_table);

            let ptr = pixels.values().as_ptr();
            let len = pixels.len();

            let pixels = Vec::from_raw_parts(ptr as *mut u8, len, len);

            return match encoding {
                Encoding::RGB8 => Self::new_rgb8(pixels, width, height, name),
                Encoding::BGR8 => Self::new_bgr8(pixels, width, height, name),
                Encoding::Unknown => panic!("Unknown encoding"),
            };
        }
    }

    pub fn to_arrow(self) -> arrow::array::UnionArray {
        let pixels = arrow::array::UInt8Array::from(self.pixels);

        let width = arrow::array::UInt32Array::from(vec![self.width; 1]);
        let height = arrow::array::UInt32Array::from(vec![self.height; 1]);
        let encoding = arrow::array::StringArray::from(vec![self.encoding.to_string(); 1]);

        let name = arrow::array::StringArray::from(vec![self.name; 1]);

        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

        let union_fields = [
            union_field(0, "pixels", arrow::datatypes::DataType::UInt8, false),
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

        arrow::array::UnionArray::try_new(union_fields, type_ids, Some(offsets), children).unwrap()
    }
}
