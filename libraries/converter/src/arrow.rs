use viewer::ArrowDataViewer;

pub mod builder;
pub mod consumer;
pub mod viewer;

pub trait IntoArrow {
    fn into_arrow(self) -> eyre::Result<arrow::array::ArrayData>;
    fn from_arrow(array_data: arrow::array::ArrayData) -> eyre::Result<Self>
    where
        Self: Sized;
}

pub trait ViewArrow<'a> {
    fn viewer(array_data: arrow::array::ArrayData) -> eyre::Result<ArrowDataViewer>;
    fn view_arrow(viewer: &'a ArrowDataViewer) -> eyre::Result<Self>
    where
        Self: Sized;
}
