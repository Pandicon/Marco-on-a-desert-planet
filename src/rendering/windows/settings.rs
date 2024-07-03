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
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.marco_min_velocity).speed(0.01)).changed();
				ui.label("Marco's minimum velocity (m/s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.marco_max_velocity).speed(0.01)).changed();
				ui.label("Marco's maximum velocity (m/s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.velocities_count).speed(0.1)).changed();
				self.settings.velocities_count = self.settings.velocities_count.max(1);
				ui.label("Marco's velocities to simulate (count)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.drag_angle(&mut self.settings.start_lat).changed();
				self.settings.start_lat = self.settings.start_lat.clamp(-PI / 2.0, PI / 2.0);
				ui.label("Starting latitude (deg)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.drag_angle(&mut self.settings.start_lon).changed();
				self.settings.start_lon = self.settings.start_lon.clamp(-PI / 2.0, PI / 2.0);
				ui.label("Starting longitude (deg)");
			});
			ui.separator();
			ui.heading("Planet parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.planet_radius)).changed();
				self.settings.planet_radius = self.settings.planet_radius.max(10.0_f64.powi(-6));
				ui.label("Planet radius (km)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.drag_angle(&mut self.settings.rotational_axis_tilt).changed();
				self.settings.rotational_axis_tilt = self.settings.rotational_axis_tilt.clamp(-PI, PI);
				ui.label("Rotational axis tilt (deg)").on_hover_text("The tilt of the rotational axis of the planet, measured from the normal to the ecliptic. Positive values make it point towards the star at the start, negative away.");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.rotational_period)).changed();
				self.settings.rotational_period = self.settings.rotational_period.max(10.0_f64.powi(-6));
				ui.label("Sideric rotation period (h)");
			});
			ui.separator();
			ui.heading("Planet orbit parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.sun_distance)).changed();
				self.settings.sun_distance = self.settings.sun_distance.max(10.0_f64.powi(-6));
				ui.label("Semi-major axis (km)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.orbital_period).speed(0.01)).changed();
				ui.label("Orbital period (years)");
			});
			ui.separator();
			ui.heading("Simulation parameters");
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.timestep).speed(0.01)).changed();
				ui.label("Timestep (s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.simulation_time).speed(100.0)).changed();
				ui.label("Time to simulate (s)");
			});
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.points_to_show)).changed();
				ui.label("Number of points to show in the graph per simulated velocity (approximate value, usually ± 1)");
			});
			ui.separator();
			ui.heading("Image options");
			anything_changed |= ui.checkbox(&mut self.settings.generate_image, "Generate the path image").on_hover_text("The image is really great for visualisation, but takes a while to generate which is not great when playing with the parameters.").changed();
			ui.horizontal(|ui| {
				anything_changed |= ui.add(egui::DragValue::new(&mut self.settings.image_scale_factor)).changed();
				self.settings.image_scale_factor = self.settings.image_scale_factor.max(0.01);
				ui.label("Image scale factor").on_hover_text("This sets the resolution of the image, the default is 1024 by 760 pixels. This default is then multiplied on both of these axes by the scale factor.");
			});

			if self.settings.recalculate_on_change && anything_changed {
				self.recalculate();
			}
		});
        self.windows_opened.settings = opened;
    }
}
