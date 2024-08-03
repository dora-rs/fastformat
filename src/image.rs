use eyre::{Report, Result};

mod bgr8;
mod gray8;
mod rgb8;

mod arrow;

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

    pub fn to_rgb8(self) -> Result<Self> {
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
            _ => Err(Report::msg("Can't convert image to RGB8")),
        }
    }

    pub fn to_bgr8(self) -> Result<Self> {
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
