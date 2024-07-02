#[derive(Clone, Copy)]
pub struct Settings {
    pub recalculate_on_change: bool,
    pub generate_image: bool,

    pub marco_min_velocity: f64,
    pub marco_max_velocity: f64,
    pub velocities_count: usize,
    pub start_lat: f32,
    pub start_lon: f32,

    pub planet_radius: f64,
    pub rotational_axis_tilt: f32,
    pub rotational_period: f64,

    pub sun_distance: f64,
    pub orbital_period: f64,

    pub timestep: f64,
    pub simulation_time: f64,
    pub points_to_show: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recalculate_on_change: false,
            generate_image: true,

            marco_min_velocity: 0.5 / 3.6,
            marco_max_velocity: 15.0 / 3.6,
            velocities_count: 1,
            start_lat: 89.7_f32.to_radians(),
            start_lon: -90.0_f32.to_radians(),

            planet_radius: 6000.0,
            rotational_axis_tilt: 23.5_f32.to_radians(),
            rotational_period: 24.0,

            sun_distance: 150.0 * 10.0_f64.powi(6),
            orbital_period: 1.0,

            timestep: 1.0,
            simulation_time: 86400.0,
            points_to_show: 1000,
        }
    }
}
