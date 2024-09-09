use std::borrow::Cow;

use super::LaserScan2D;
use fastformat_converter::arrow::{FastFormatArrowBuilder, FastFormatArrowRawData};

use eyre::Result;

impl<'a> LaserScan2D<'a> {
    pub fn raw_data(array_data: arrow::array::ArrayData) -> Result<FastFormatArrowRawData> {
        use arrow::datatypes::{Float32Type, UInt64Type};

        let raw_data = FastFormatArrowRawData::new(array_data)?
            .load_primitive::<Float32Type>("data")?
            .load_primitive::<Float32Type>("intensities")?
            .load_primitive::<UInt64Type>("length")?
            .load_primitive::<Float32Type>("min_distance")?
            .load_primitive::<Float32Type>("max_distance")?
            .load_primitive::<Float32Type>("angle_increment")?
            .load_primitive::<Float32Type>("angle_min")?
            .load_primitive::<Float32Type>("angle_max")?;

        Ok(raw_data)
    }

    pub fn from_raw_data(mut raw_data: FastFormatArrowRawData) -> Result<Self> {
        use arrow::datatypes::{Float32Type, UInt64Type};

        let data = raw_data.primitive_array::<Float32Type>("data")?;
        let intensities = raw_data.primitive_array::<Float32Type>("intensities")?;
        let length = raw_data.primitive_singleton::<UInt64Type>("length")?;
        let min_distance = raw_data.primitive_singleton::<Float32Type>("min_distance")?;
        let max_distance = raw_data.primitive_singleton::<Float32Type>("max_distance")?;
        let angle_increment = raw_data.primitive_singleton::<Float32Type>("angle_increment")?;
        let angle_min = raw_data.primitive_singleton::<Float32Type>("angle_min")?;
        let angle_max = raw_data.primitive_singleton::<Float32Type>("angle_max")?;

        Ok(Self {
            data: Cow::Owned(data),
            intensities: Cow::Owned(intensities),
            length,
            min_distance,
            max_distance,
            angle_increment,
            angle_min,
            angle_max,
        })
    }

    pub fn view_from_raw_data(raw_data: &'a FastFormatArrowRawData) -> Result<Self> {
        use arrow::datatypes::{Float32Type, UInt64Type};

        let data = raw_data.primitive_array_view::<Float32Type>("data")?;
        let intensities = raw_data.primitive_array_view::<Float32Type>("intensities")?;
        let length = raw_data.primitive_singleton::<UInt64Type>("length")?;
        let min_distance = raw_data.primitive_singleton::<Float32Type>("min_distance")?;
        let max_distance = raw_data.primitive_singleton::<Float32Type>("max_distance")?;
        let angle_increment = raw_data.primitive_singleton::<Float32Type>("angle_increment")?;
        let angle_min = raw_data.primitive_singleton::<Float32Type>("angle_min")?;
        let angle_max = raw_data.primitive_singleton::<Float32Type>("angle_max")?;

        Ok(Self {
            data: Cow::Borrowed(data),
            intensities: Cow::Borrowed(intensities),
            length,
            min_distance,
            max_distance,
            angle_increment,
            angle_min,
            angle_max,
        })
    }

    pub fn from_arrow(array_data: arrow::array::ArrayData) -> Result<Self> {
        Self::from_raw_data(Self::raw_data(array_data)?)
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::datatypes::{
            DataType::{Float32, UInt64},
            Float32Type, UInt64Type,
        };

        let raw_data = FastFormatArrowBuilder::new()
            .push_primitive_array::<Float32Type>("data", self.data.into_owned(), Float32, false)
            .push_primitive_array::<Float32Type>(
                "intensities",
                self.intensities.into_owned(),
                Float32,
                false,
            )
            .push_primitive_singleton::<UInt64Type>("length", self.length, UInt64, false)
            .push_primitive_singleton::<Float32Type>(
                "min_distance",
                self.min_distance,
                Float32,
                false,
            )
            .push_primitive_singleton::<Float32Type>(
                "max_distance",
                self.max_distance,
                Float32,
                false,
            )
            .push_primitive_singleton::<Float32Type>(
                "angle_increment",
                self.angle_increment,
                Float32,
                false,
            )
            .push_primitive_singleton::<Float32Type>("angle_min", self.angle_min, Float32, false)
            .push_primitive_singleton::<Float32Type>("angle_max", self.angle_max, Float32, false);

        raw_data.into_arrow()
    }
}

mod tests {}
