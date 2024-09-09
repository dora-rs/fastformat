use eyre::Result;

mod arrow;

#[derive(Debug)]
pub struct ImageInVideo {
    pub video_path: String,
    pub timestamp: f32,
    pub framerate: f32,
    pub name: Option<String>,
}

impl ImageInVideo {
    pub fn new(
        video_path: String,
        timestamp: f32,
        framerate: f32,
        name: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            video_path,
            timestamp,
            framerate,
            name,
        })
    }
}

mod tests {}
