use std::f32::consts::PI;

use eframe::egui;

use crate::application;

impl application::Application {
    pub fn render_settings(&mut self, ctx: &egui::Context) {
        let mut opened = self.windows_opened.settings;
        egui::Window::new("Settings").open(&mut opened).show(ctx, |ui| {
			let mut anything_changed = false;
            ui.checkbox(&mut self.settings.recalculate_on_change, "Recalculate on change").on_hover_text("If this option is enabled the simulation will be recalculated every time any of the parameters changes. Can be great for playing with starting values, but can be computationally expensive and therefore make the application run quite slow.");
			ui.separator();
			ui.heading("Marco parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.marco_velocity)).changed();
				ui.label("Marco's velocity (m/s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.drag_angle(&mut self.settings.start_lat).changed();
				self.settings.start_lat = self.settings.start_lat.clamp(-PI/2.0, PI / 2.0);
				ui.label("Starting latitude (deg)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.drag_angle(&mut self.settings.start_lon).changed();
				self.settings.start_lon = self.settings.start_lon.clamp(-PI/2.0, PI / 2.0);
				ui.label("Starting longitude (deg)");
			});
			ui.separator();
			ui.heading("Planet parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.planet_radius)).changed();
				self.settings.planet_radius = self.settings.planet_radius.max(10.0_f64.powi(-6));
				ui.label("Planet radius (km)");
			});
			ui.separator();
			ui.heading("Planet orbit parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.sun_distance)).changed();
				self.settings.sun_distance = self.settings.sun_distance.max(10.0_f64.powi(-6));
				ui.label("Semi-major axis (km)");
			});
			ui.separator();
			ui.heading("Simulation parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.timestep)).changed();
				ui.label("Timestep (s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.simulation_time)).changed();
				ui.label("Time to simulate (s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.points_to_show)).changed();
				ui.label("Number of points to show in the graph (approximate value, usually ± 1");
			});
			if self.settings.recalculate_on_change && anything_changed {
				self.recalculate();
			}
		});
        self.windows_opened.settings = opened;
    }
}
