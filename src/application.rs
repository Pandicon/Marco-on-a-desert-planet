use std::f64::consts::PI;

use nalgebra::Vector3;

use eframe::egui;

use crate::{data, settings};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WindowToShow {
    LatitudeVsTimeGraph,
    LongitudeVsTimeGraph,
}

pub struct Application {
    window_to_show: WindowToShow,
    pub data: Vec<data::Data>,
    pub windows_opened: WindowsOpened,
    pub settings: settings::Settings,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            window_to_show: WindowToShow::LatitudeVsTimeGraph,
            data: Vec::new(),
            windows_opened: WindowsOpened::default(),
            settings: settings::Settings::default(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.selectable_value(&mut self.window_to_show, WindowToShow::LatitudeVsTimeGraph, "Graph of latitude vs time");
                    ui.selectable_value(&mut self.window_to_show, WindowToShow::LongitudeVsTimeGraph, "Graph of longitude vs time");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Recalculate").clicked() {
                        self.recalculate();
                    }
                    if ui.button("Settings").clicked() {
                        self.windows_opened.settings = true;
                    }
                });
            });
        });
        self.render_settings(ctx);
        egui::CentralPanel::default().show(ctx, |ui| match self.window_to_show {
            WindowToShow::LatitudeVsTimeGraph => self.render_latitude_vs_time_graph(ui),
            WindowToShow::LongitudeVsTimeGraph => self.render_longitude_vs_time_graph(ui),
        });
        ctx.request_repaint();
    }
}

impl Application {
    pub fn recalculate(&mut self) {
        let mut time = 0.0;
        let start_lat = self.settings.start_lat as f64;
        let start_lon = self.settings.start_lon as f64;
        let axis_tilt = PI / 2.0 - self.settings.rotational_axis_tilt as f64;

        let planet_rotation_axis = Vector3::new(axis_tilt.cos(), 0.0, axis_tilt.sin()).normalize();
        let planet_rotation_quaternion = nalgebra::UnitQuaternion::new(planet_rotation_axis * (2.0 * PI) / (self.settings.rotational_period * 3600.0) * self.settings.timestep * (-1.0)); // Multiplied by -1 to make the star orbit the planet in the correct direction

        let mut marco_pos = Vector3::new(start_lat.cos() * start_lon.cos(), start_lat.cos() * start_lon.sin(), start_lat.sin()) * self.settings.planet_radius;
        let mut sun_pos_norm = Vector3::new(1.0_f64, 0.0, 0.0);

        self.data = Vec::new();
        self.data.push(data::Data::from_raw(marco_pos, time, self.settings.planet_radius));

        while time <= self.settings.simulation_time {
            let marco_to_sun = (self.settings.sun_distance * sun_pos_norm - marco_pos).normalize();

            // Only move Marco when the star is above his horizon
            if marco_to_sun.dot(&marco_pos) >= 0.0 {
                let rotation_axis = (marco_pos.normalize().cross(&marco_to_sun)).normalize() * (self.settings.marco_velocity / 1000.0 * self.settings.timestep) / self.settings.planet_radius;
                let rotation_quaternion = nalgebra::UnitQuaternion::new(rotation_axis);

                marco_pos = rotation_quaternion * marco_pos;
            }

            sun_pos_norm = planet_rotation_quaternion * sun_pos_norm;

            if time / self.settings.simulation_time > (self.data.len() as f64) / (self.settings.points_to_show as f64) {
                self.data.push(data::Data::from_raw(marco_pos, time, self.settings.planet_radius));
            }

            time += self.settings.timestep;
        }
        self.data.push(data::Data::from_raw(marco_pos, time, self.settings.planet_radius));
    }
}

#[derive(Default)]
pub struct WindowsOpened {
    pub settings: bool,
}
