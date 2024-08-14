use super::{data::ImageData, encoding::Encoding, Image};

use eyre::Result;

use crate::arrow::{RawData, UnionBuilder};

impl<'a> Image<'a> {
    pub fn raw_data(array_data: arrow::array::ArrayData) -> Result<RawData> {
        use arrow::datatypes::{UInt32Type, UInt8Type};

        let raw_data = RawData::new(array_data)?
            .load_primitive::<UInt32Type>("width")?
            .load_primitive::<UInt32Type>("height")?
            .load_utf("encoding")?
            .load_utf("name")?;

        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;
        let raw_data = match encoding {
            Encoding::RGB8 => raw_data.load_primitive::<UInt8Type>("data")?,
            Encoding::BGR8 => raw_data.load_primitive::<UInt8Type>("data")?,
            Encoding::GRAY8 => raw_data.load_primitive::<UInt8Type>("data")?,
        };

        Ok(raw_data)
    }

    pub fn from_raw_data(mut raw_data: RawData) -> Result<Self> {
        use arrow::datatypes::{UInt32Type, UInt8Type};

        let width = raw_data.primitive_singleton::<UInt32Type>("width")?;
        let height = raw_data.primitive_singleton::<UInt32Type>("height")?;
        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;
        let name = Some(raw_data.utf8_singleton("name")?).filter(|s| !s.is_empty());

        let data = match encoding {
            Encoding::RGB8 => raw_data.primitive_array::<UInt8Type>("data")?,
            Encoding::BGR8 => raw_data.primitive_array::<UInt8Type>("data")?,
            Encoding::GRAY8 => raw_data.primitive_array::<UInt8Type>("data")?,
        };

        Ok(Self {
            data: ImageData::from_vec_u8(data),
            width,
            height,
            encoding,
            name,
        })
    }

    pub fn view_from_raw_data(raw_data: &'a RawData) -> Result<Self> {
        use arrow::datatypes::{UInt32Type, UInt8Type};

        let width = raw_data.primitive_singleton::<UInt32Type>("width")?;
        let height = raw_data.primitive_singleton::<UInt32Type>("height")?;
        let encoding = Encoding::from_string(raw_data.utf8_singleton("encoding")?)?;
        let name = Some(raw_data.utf8_singleton("name")?).filter(|s| !s.is_empty());

        let data = match encoding {
            Encoding::RGB8 => raw_data.primitive_array_view::<UInt8Type>("data")?,
            Encoding::BGR8 => raw_data.primitive_array_view::<UInt8Type>("data")?,
            Encoding::GRAY8 => raw_data.primitive_array_view::<UInt8Type>("data")?,
        };

        Ok(Self {
            data: ImageData::from_slice_u8(data),
            width,
            height,
            encoding,
            name,
        })
    }

    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_data(Self::raw_data(array_data)?)
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::datatypes::{
            DataType::{UInt32, UInt8, Utf8},
            UInt32Type, UInt8Type,
        };

        let raw_data = UnionBuilder::new()
            .push_primitive_singleton::<UInt32Type>("width", self.width, UInt32, false)
            .push_primitive_singleton::<UInt32Type>("height", self.height, UInt32, false)
            .push_utf_singleton("encoding", self.encoding.to_string(), Utf8, false)
            .push_utf_singleton(
                "name",
                self.name.map_or_else(|| "".to_string(), |s| s),
                Utf8,
                false,
            );

        let raw_data = match self.encoding {
            Encoding::RGB8 => raw_data.push_primitive_array::<UInt8Type>(
                "data",
                self.data.into_u8()?,
                UInt8,
                false,
            ),
            Encoding::BGR8 => raw_data.push_primitive_array::<UInt8Type>(
                "data",
                self.data.into_u8()?,
                UInt8,
                false,
            ),
            Encoding::GRAY8 => raw_data.push_primitive_array::<UInt8Type>(
                "data",
                self.data.into_u8()?,
                UInt8,
                false,
            ),
        };

        raw_data.into_arrow()
    }
}

mod tests {
    #[test]
    fn test_arrow_zero_copy_conversion() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let bgr8_image = Image::from_arrow(arrow_image).unwrap();
        let bgr8_image_buffer = bgr8_image.data.as_ptr();

        let rgb8_image = bgr8_image.into_rgb8().unwrap();
        let rgb8_image_buffer = rgb8_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, bgr8_image_buffer);
        assert_eq!(bgr8_image_buffer, rgb8_image_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_read_only() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let raw_data = Image::raw_data(arrow_image).unwrap();
        let new_image = Image::view_from_raw_data(&raw_data).unwrap();

        let final_image_buffer = new_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_eq!(image_buffer_address, final_image_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_copy_on_write() {
        use crate::image::Image;

        let flat_image = vec![0; 27];
        let original_buffer_address = flat_image.as_ptr() as *const u64;

        let bgr8_image = Image::new_bgr8(flat_image, 3, 3, None).unwrap();
        let image_buffer_address = bgr8_image.data.as_ptr();

        let arrow_image = bgr8_image.into_arrow().unwrap();

        let raw_data = Image::raw_data(arrow_image).unwrap();
        let bgr8_image = Image::view_from_raw_data(&raw_data).unwrap();
        let rgb8_image = bgr8_image.into_rgb8().unwrap();

        let final_image_buffer = rgb8_image.data.as_ptr();

        assert_eq!(original_buffer_address, image_buffer_address);
        assert_ne!(image_buffer_address, final_image_buffer);
    }
}
