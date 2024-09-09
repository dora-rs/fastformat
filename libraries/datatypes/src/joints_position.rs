use encoding::Encoding;
use eyre::Result;

mod arrow;
mod encoding;

#[derive(Debug)]
pub struct JointsPosition {
    pub joints: Vec<String>,
    pub positions: Vec<f32>,
    pub encoding: Encoding,
}

impl JointsPosition {
    pub fn new(joints: Vec<String>, positions: Vec<f32>, encoding: Encoding) -> Result<Self> {
        Ok(Self {
            joints,
            positions,
            encoding,
        })
    }
}

mod tests {}
