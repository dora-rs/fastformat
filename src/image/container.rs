use eyre::Result;

#[derive(Debug)]
pub enum DataContainer {
    U8Data(Vec<u8>),
}

impl DataContainer {
    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Self::U8Data(data) => data.as_ptr(),
        }
    }

    pub fn into_u8(self) -> Result<Vec<u8>> {
        match self {
            Self::U8Data(data) => Ok(data),
        }
    }

    pub fn to_u8(&self) -> Result<&Vec<u8>> {
        match self {
            Self::U8Data(data) => Ok(data),
        }
    }

    pub fn to_mut_u8(&mut self) -> Result<&mut Vec<u8>> {
        match self {
            Self::U8Data(data) => Ok(data),
        }
    }

    pub fn from_u8(data: Vec<u8>) -> Self {
        Self::U8Data(data)
    }
}
