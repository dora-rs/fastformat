use std::borrow::Cow;

use fastformat::prelude::*;

#[derive(Debug)]
pub struct CustomDataType {
    size: u32,
    label: String,
    ranges: Vec<u8>,
}

#[derive(Debug)]
pub struct CustomDataTypeView<'a> {
    size: u32,
    label: String,
    ranges: Cow<'a, [u8]>,
}

impl IntoArrow for CustomDataType {
    fn into_arrow(self) -> eyre::Result<arrow::array::ArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<arrow::datatypes::UInt32Type>("size", self.size)
            .push_utf8_singleton("label", self.label)
            .push_primitive_array::<arrow::datatypes::UInt8Type>("ranges", self.ranges);

        builder.build()
    }

    fn from_arrow(array_data: arrow::array::ArrayData) -> eyre::Result<Self> {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let size = consumer.primitive_singleton::<arrow::datatypes::UInt32Type>("size")?;
        let label = consumer.utf8_singleton("label")?;
        let ranges = consumer.primitive_array::<arrow::datatypes::UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges,
        })
    }
}

impl IntoArrow for CustomDataTypeView<'_> {
    fn into_arrow(self) -> eyre::Result<arrow::array::ArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<arrow::datatypes::UInt32Type>("size", self.size)
            .push_utf8_singleton("label", self.label)
            .push_primitive_array::<arrow::datatypes::UInt8Type>(
                "ranges",
                self.ranges.into_owned(),
            );

        builder.build()
    }

    fn from_arrow(array_data: arrow::array::ArrayData) -> eyre::Result<Self> {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let size = consumer.primitive_singleton::<arrow::datatypes::UInt32Type>("size")?;
        let label = consumer.utf8_singleton("label")?;
        let ranges = consumer.primitive_array::<arrow::datatypes::UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges: Cow::Owned(ranges),
        })
    }
}

impl<'a> ViewArrow<'a> for CustomDataTypeView<'a> {
    fn viewer(array_data: arrow::array::ArrayData) -> eyre::Result<ArrowDataViewer> {
        ArrowDataViewer::new(array_data)?.load_primitive::<arrow::datatypes::UInt8Type>("ranges")
    }
    fn view_arrow(viewer: &'a ArrowDataViewer) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let size = viewer.primitive_singleton::<arrow::datatypes::UInt32Type>("size")?;
        let label = viewer.utf8_singleton("label")?;
        let ranges = viewer.primitive_array::<arrow::datatypes::UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges: Cow::Borrowed(ranges),
        })
    }
}

fn main() -> eyre::Result<()> {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ptr = data.as_ptr();

    let custom_data = CustomDataType {
        size: 10,
        label: "Custom".to_string(),
        ranges: data,
    };

    let arrow_data = custom_data.into_arrow()?;
    let custom_data = CustomDataType::from_arrow(arrow_data)?;

    println!("{:?}", custom_data);

    let arrow_data = custom_data.into_arrow()?;
    let viewer = CustomDataTypeView::viewer(arrow_data)?;

    let custom_data = CustomDataTypeView::view_arrow(&viewer)?;
    let ptr2 = custom_data.ranges.as_ptr();

    println!("{:?}", custom_data);

    println!("ptr1: {:p}, ptr2: {:p}", ptr, ptr2);

    Ok(())
}
