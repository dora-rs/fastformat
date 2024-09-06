use eyre::Result;

#[derive(Debug)]
pub enum Ndarray {
    F32IX1(ndarray::Array<f32, ndarray::Ix1>),
    U8IX2(ndarray::Array<u8, ndarray::Ix2>),
    U8IX3(ndarray::Array<u8, ndarray::Ix3>),
    STRIX1(ndarray::Array<String, ndarray::Ix1>),
}

impl Ndarray {
    pub fn as_ptr(&self) -> *const u64 {
        match self {
            Ndarray::F32IX1(array) => array.as_ptr() as *const u64,
            Ndarray::U8IX2(array) => array.as_ptr() as *const u64,
            Ndarray::U8IX3(array) => array.as_ptr() as *const u64,
            Ndarray::STRIX1(array) => array.as_ptr() as *const u64,
        }
    }

    pub fn into_u8_ix3(self) -> Result<ndarray::Array<u8, ndarray::Ix3>> {
        match self {
            Ndarray::U8IX3(array) => Ok(array),
            _ => Err(eyre::Report::msg("Expected U8IX3")),
        }
    }

    pub fn into_u8_ix2(self) -> Result<ndarray::Array<u8, ndarray::Ix2>> {
        match self {
            Ndarray::U8IX2(array) => Ok(array),
            _ => Err(eyre::Report::msg("Expected U8IX2")),
        }
    }

    pub fn into_f32_ix1(self) -> Result<ndarray::Array<f32, ndarray::Ix1>> {
        match self {
            Ndarray::F32IX1(array) => Ok(array),
            _ => Err(eyre::Report::msg("Expected F32IX1")),
        }
    }
}

#[derive(Debug)]
pub enum NdarrayView<'a> {
    F32IX1(ndarray::ArrayView<'a, f32, ndarray::Ix1>),
    U8IX2(ndarray::ArrayView<'a, u8, ndarray::Ix2>),
    U8IX3(ndarray::ArrayView<'a, u8, ndarray::Ix3>),
    STRIX1(ndarray::ArrayView<'a, String, ndarray::Ix1>),
}

impl<'a> NdarrayView<'a> {
    pub fn as_ptr(&self) -> *const u64 {
        match self {
            NdarrayView::F32IX1(array) => array.as_ptr() as *const u64,
            NdarrayView::U8IX2(array) => array.as_ptr() as *const u64,
            NdarrayView::U8IX3(array) => array.as_ptr() as *const u64,
            NdarrayView::STRIX1(array) => array.as_ptr() as *const u64,
        }
    }
}

#[derive(Debug)]
pub enum NdarrayViewMut<'a> {
    F32IX1(ndarray::ArrayViewMut<'a, f32, ndarray::Ix1>),
    U8IX2(ndarray::ArrayViewMut<'a, u8, ndarray::Ix2>),
    U8IX3(ndarray::ArrayViewMut<'a, u8, ndarray::Ix3>),
    STRIX1(ndarray::ArrayViewMut<'a, String, ndarray::Ix1>),
}

impl NdarrayViewMut<'_> {
    pub fn as_ptr(&self) -> *const u64 {
        match self {
            NdarrayViewMut::F32IX1(array) => array.as_ptr() as *const u64,
            NdarrayViewMut::U8IX2(array) => array.as_ptr() as *const u64,
            NdarrayViewMut::U8IX3(array) => array.as_ptr() as *const u64,
            NdarrayViewMut::STRIX1(array) => array.as_ptr() as *const u64,
        }
    }
}
