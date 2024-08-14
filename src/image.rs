use eyre::{Report, Result};

use data::ImageData;
use encoding::Encoding;

mod bgr8;
mod gray8;
mod rgb8;

mod arrow;
mod data;
mod encoding;

#[derive(Debug)]
pub struct Image<'a> {
    pub data: ImageData<'a>,

    pub width: u32,
    pub height: u32,

    pub encoding: Encoding,

    pub name: Option<String>,
}

impl Image<'_> {
    pub fn into_rgb8(self) -> Result<Self> {
        match self.encoding {
            Encoding::BGR8 => {
                let mut data = self.data.into_u8()?;

                for i in (0..data.len()).step_by(3) {
                    data.swap(i, i + 2);
                }
                Ok(Image {
                    data: ImageData::from_vec_u8(data),
                    width: self.width,
                    height: self.height,
                    encoding: Encoding::RGB8,
                    name: self.name.clone(),
                })
            }
            Encoding::RGB8 => Ok(self),
            _ => Err(Report::msg("Can't convert image to RGB8")),
        }
    }

    pub fn into_bgr8(self) -> Result<Self> {
        match self.encoding {
            Encoding::RGB8 => {
                let mut data = self.data.into_u8()?;

                for i in (0..data.len()).step_by(3) {
                    data.swap(i, i + 2);
                }

                Ok(Image {
                    data: ImageData::from_vec_u8(data),
                    width: self.width,
                    height: self.height,
                    encoding: Encoding::BGR8,
                    name: self.name.clone(),
                })
            }
            Encoding::BGR8 => Ok(self),
            _ => Err(Report::msg("Can't convert image to BGR8")),
        }
    }
}

mod tests {
    #[test]
    fn test_rgb8_into_bgr8() {
        use crate::image::Image;

        let flat_image = (0..27).collect::<Vec<u8>>();
        let image = Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();

        let final_image = image.into_bgr8().unwrap();
        let final_image_data = final_image.data.as_u8().unwrap();

        let expected_image = vec![
            2, 1, 0, 5, 4, 3, 8, 7, 6, 11, 10, 9, 14, 13, 12, 17, 16, 15, 20, 19, 18, 23, 22, 21,
            26, 25, 24,
        ];

        assert_eq!(&expected_image, final_image_data);
    }

    #[test]
    fn test_bgr8_into_rgb8() {
        use crate::image::Image;

        let flat_image = (0..27).collect::<Vec<u8>>();
        let image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();

        let final_image = image.into_rgb8().unwrap();
        let final_image_data = final_image.data.as_u8().unwrap();

        let expected_image = vec![
            2, 1, 0, 5, 4, 3, 8, 7, 6, 11, 10, 9, 14, 13, 12, 17, 16, 15, 20, 19, 18, 23, 22, 21,
            26, 25, 24,
        ];

        assert_eq!(&expected_image, final_image_data);
    }
}
