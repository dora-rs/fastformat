use eyre::Result;

use std::borrow::Cow;

#[derive(Debug)]
pub enum ImageData<'a> {
    U8(Cow<'a, [u8]>),
    U16(Cow<'a, [u16]>),
    F32(Cow<'a, [f32]>),
}

impl ImageData<'_> {
    pub fn len(&self) -> usize {
        match self {
            Self::U8(data) => data.len(),
            Self::U16(data) => data.len(),
            Self::F32(data) => data.len(),
        }
    }

    pub fn as_ptr(&self) -> *const u64 {
        match self {
            Self::U8(data) => data.as_ptr() as *const u64,
            Self::U16(data) => data.as_ptr() as *const u64,
            Self::F32(data) => data.as_ptr() as *const u64,
        }
    }

    pub fn into_u8(self) -> Result<Vec<u8>> {
        match self {
            Self::U8(data) => Ok(data.into_owned()),
            _ => Err(eyre::Report::msg("Can't convert data to u8")),
        }
    }

    pub fn into_u16(self) -> Result<Vec<u16>> {
        match self {
            Self::U16(data) => Ok(data.into_owned()),
            _ => Err(eyre::Report::msg("Can't convert data to u16")),
        }
    }

    pub fn into_f32(self) -> Result<Vec<f32>> {
        match self {
            Self::F32(data) => Ok(data.into_owned()),
            _ => Err(eyre::Report::msg("Can't convert data to f32")),
        }
    }

    pub fn as_u8(&self) -> Result<&[u8]> {
        match self {
            Self::U8(data) => Ok(data),
            _ => Err(eyre::Report::msg("Can't convert data to u8")),
        }
    }

    pub fn as_u16(&self) -> Result<&[u16]> {
        match self {
            Self::U16(data) => Ok(data),
            _ => Err(eyre::Report::msg("Can't convert data to u16")),
        }
    }

    pub fn as_f32(&self) -> Result<&[f32]> {
        match self {
            Self::F32(data) => Ok(data),
            _ => Err(eyre::Report::msg("Can't convert data to f32")),
        }
    }

    pub fn as_mut_u8(&mut self) -> Result<&mut Vec<u8>> {
        match self {
            Self::U8(data) => Ok(data.to_mut()),
            _ => Err(eyre::Report::msg("Can't convert data to u8")),
        }
    }

    pub fn as_mut_u16(&mut self) -> Result<&mut Vec<u16>> {
        match self {
            Self::U16(data) => Ok(data.to_mut()),
            _ => Err(eyre::Report::msg("Can't convert data to 16")),
        }
    }

    pub fn as_mut_f32(&mut self) -> Result<&mut Vec<f32>> {
        match self {
            Self::F32(data) => Ok(data.to_mut()),
            _ => Err(eyre::Report::msg("Can't convert data to f32")),
        }
    }

    pub fn from_vec_u8(data: Vec<u8>) -> Self {
        Self::U8(Cow::from(data))
    }

    pub fn from_vec_u16(data: Vec<u16>) -> Self {
        Self::U16(Cow::from(data))
    }

    pub fn from_vec_f32(data: Vec<f32>) -> Self {
        Self::F32(Cow::from(data))
    }
}

impl<'a> ImageData<'a> {
    pub fn from_slice_u8(data: &'a [u8]) -> Self {
        Self::U8(Cow::from(data))
    }

    pub fn from_slice_u16(data: &'a [u16]) -> Self {
        Self::U16(Cow::from(data))
    }

    pub fn from_slice_f32(data: &'a [f32]) -> Self {
        Self::F32(Cow::from(data))
    }
}
