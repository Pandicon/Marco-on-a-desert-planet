pub struct Settings {
    pub recalculate_on_change: bool,

    pub marco_velocity: f64,
    pub planet_radius: f64,
    pub sun_distance: f64,
    pub start_lat: f32,
    pub start_lon: f32,

    pub timestep: f64,
    pub simulation_time: f64,
    pub points_to_show: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recalculate_on_change: false,
            marco_velocity: 5.0 / 3.6,
            planet_radius: 6000.0,
            sun_distance: 150.0 * 10.0_f64.powi(6),
            start_lat: 89.7_f32.to_radians(),
            start_lon: -50.0_f32.to_radians(),
            timestep: 1.0,
            simulation_time: 86400.0,
            points_to_show: 1000,
        }
    }
}
