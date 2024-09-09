use super::ImageInVideo;
use eyre::Result;
use fastformat_converter::arrow::{FastFormatArrowBuilder, FastFormatArrowRawData};

impl ImageInVideo {
    pub fn raw_data(array_data: arrow::array::ArrayData) -> Result<FastFormatArrowRawData> {
        use arrow::datatypes::Float32Type;

        let raw_data = FastFormatArrowRawData::new(array_data)?
            .load_utf("video_path")?
            .load_primitive::<Float32Type>("timestamp")?
            .load_primitive::<Float32Type>("framerate")?
            .load_utf("name")?;

        Ok(raw_data)
    }

    pub fn from_raw_data(raw_data: FastFormatArrowRawData) -> Result<Self> {
        use arrow::datatypes::Float32Type;

        let video_path = raw_data.utf8_singleton("video_path")?;
        let timestamp = raw_data.primitive_singleton::<Float32Type>("timestamp")?;
        let framerate = raw_data.primitive_singleton::<Float32Type>("framerate")?;
        let name = Some(raw_data.utf8_singleton("name")?).filter(|s| !s.is_empty());

        Self::new(video_path, timestamp, framerate, name)
    }

    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_data(Self::raw_data(array_data)?)
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::datatypes::{
            DataType::{Float32, Utf8},
            Float32Type,
        };

        let raw_data = FastFormatArrowBuilder::new()
            .push_utf_singleton("video_path", self.video_path, Utf8, false)
            .push_primitive_singleton::<Float32Type>("timestamp", self.timestamp, Float32, false)
            .push_primitive_singleton::<Float32Type>("framerate", self.framerate, Float32, false)
            .push_utf_singleton(
                "name",
                self.name.map_or_else(|| "".to_string(), |s| s),
                Utf8,
                false,
            );

        raw_data.into_arrow()
    }
}

mod tests {}
