use nalgebra::Vector3;

#[derive(Default, Clone, Debug)]
pub struct Data {
    pub latitude: f64,
    pub longitude: f64,
    pub time: f64,
    pub colour: eframe::egui::Color32,
}

impl Data {
    pub fn new(latitude: f64, longitude: f64, time: f64, colour: eframe::egui::Color32) -> Self {
        Self { latitude, longitude, time, colour }
    }

    pub fn from_raw(marco_pos: Vector3<f64>, time: f64, planet_radius: f64, colour: eframe::egui::Color32) -> Self {
        Self::new((marco_pos.z / planet_radius).asin().to_degrees(), marco_pos.y.atan2(marco_pos.x).to_degrees(), time, colour)
    }
}
