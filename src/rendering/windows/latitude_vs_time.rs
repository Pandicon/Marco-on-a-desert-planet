use std::ops::RangeInclusive;

use eframe::egui;
use egui_plot::{self, GridMark};

use crate::application;

impl application::Application {
    pub fn render_latitude_vs_time_graph(&self, ui: &mut egui::Ui) {
        let x_fmt = |x: GridMark, _max_len: usize, _range: &RangeInclusive<f64>| format!("{:.3} s", x.value);

        let y_fmt = |y: GridMark, _max_len: usize, _range: &RangeInclusive<f64>| format!("{:.3} deg", y.value);

        let label_fmt = |_s: &str, val: &egui_plot::PlotPoint| format!("{:.9} s\n{:.9} deg", val.x, val.y);

        let plot = egui_plot::Plot::new("Latitude vs time relationship")
            // .data_aspect(1.0)
            .x_axis_formatter(x_fmt)
            .y_axis_formatter(y_fmt)
            .label_formatter(label_fmt)
            .legend(egui_plot::Legend::default());

        let mut all_points = Vec::new();
        let mut all_lines = Vec::new();

        for (colour, points_sequence) in self.data.iter() {
            let points_raw = points_sequence.clone().into_iter().map(|point| [point.time, point.latitude]).collect::<Vec<[f64; 2]>>();
            let points = egui_plot::Points::new(points_raw.clone()).color(*colour).highlight(true);
            let lines = egui_plot::Line::new(egui_plot::PlotPoints::new(points_raw)).color(*colour).highlight(true);
            all_points.push(points);
            all_lines.push(lines);
        }

        plot.show(ui, |plot_ui| {
            for lines in all_lines {
                plot_ui.line(lines);
            }
            for points in all_points {
                plot_ui.points(points);
            }
        });
    }
}
