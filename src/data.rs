use nalgebra::Vector3;

#[derive(Default, Clone)]
pub struct Data {
    pub latitude: f64,
    pub longitude: f64,
    pub time: f64,
}

impl Data {
    pub fn new(latitude: f64, longitude: f64, time: f64) -> Self {
        Self { latitude, longitude, time }
    }

    pub fn from_raw(marco_pos: Vector3<f64>, time: f64, planet_radius: f64) -> Self {
        Self::new((marco_pos.z / planet_radius).asin().to_degrees(), marco_pos.y.atan2(marco_pos.x).to_degrees(), time)
    }
}
