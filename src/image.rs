use eyre::{Report, Result};

use container::DataContainer;
use encoding::Encoding;

mod bgr8;
mod gray8;
mod rgb8;

mod arrow;
mod container;
mod encoding;

#[derive(Debug)]
pub struct Image {
    data: DataContainer,

    width: u32,
    height: u32,

    encoding: Encoding,

    name: Option<String>,
}

impl Image {
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn to_rgb8(self) -> Result<Self> {
        match self.encoding {
            Encoding::BGR8 => {
                let mut data = self.data.into_u8()?;

                for i in (0..data.len()).step_by(3) {
                    data.swap(i, i + 2);
                }

                Ok(Image {
                    data: DataContainer::from_u8(data),
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

    pub fn to_bgr8(self) -> Result<Self> {
        match self.encoding {
            Encoding::RGB8 => {
                let mut data = self.data.into_u8()?;

                for i in (0..data.len()).step_by(3) {
                    data.swap(i, i + 2);
                }

                Ok(Image {
                    data: DataContainer::from_u8(data),
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
    fn test_rgb8_to_bgr8() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let image = Image::new_rgb8(flat_image, 3, 3, Some("camera.test")).unwrap();
        image.to_bgr8().unwrap();
    }

    #[test]
    fn test_bgr8_to_rgb8() {
        use crate::image::Image;

        let flat_image = vec![0; 27];

        let image = Image::new_bgr8(flat_image, 3, 3, Some("camera.test")).unwrap();
        image.to_rgb8().unwrap();
    }
}
