use eyre::Result;
use std::borrow::Cow;

mod arrow;

pub struct LaserScan2D<'a> {
    pub data: Cow<'a, [f32]>,
    pub intensities: Cow<'a, [f32]>,
    pub length: u64,
    pub min_distance: f32,
    pub max_distance: f32,
    pub angle_increment: f32,
    pub angle_min: f32,
    pub angle_max: f32,
}

impl LaserScan2D<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        data: Vec<f32>,
        intensities: Vec<f32>,
        length: u64,
        min_distance: f32,
        max_distance: f32,
        angle_increment: f32,
        angle_min: f32,
        angle_max: f32,
    ) -> Result<Self> {
        if data.len() != intensities.len() {
            return Err(eyre::Report::msg("Data and Intensities length mismatch"));
        }

        Ok(Self {
            data: Cow::from(data),
            intensities: Cow::from(intensities),
            length,
            min_distance,
            max_distance,
            angle_increment,
            angle_min,
            angle_max,
        })
    }
}

mod tests {
    #[test]
    pub fn test_laser_scan_2d_creation() {
        use crate::laser_scan_2d::LaserScan2D;

        let data = vec![0.0; 10];
        let intensities = vec![0.0; 10];

        LaserScan2D::new(data, intensities, 10, 0.0, 10.0, 0.1, -1.0, 1.0).unwrap();
    }
}
