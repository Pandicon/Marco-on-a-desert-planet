use std::collections::HashMap;

use eframe::egui;

use crate::{data, message_passers, settings, simulator};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WindowToShow {
    LatitudeVsTimeGraph,
    LongitudeVsTimeGraph,
}

pub struct Application {
    window_to_show: WindowToShow,
    pub calculation_stage: message_passers::CalculationStage,
    pub data: HashMap<egui::Color32, Vec<data::Data>>,
    pub windows_opened: WindowsOpened,
    pub settings: settings::Settings,

    pub message_passers: message_passers::MessagePassers,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            window_to_show: WindowToShow::LatitudeVsTimeGraph,
            calculation_stage: message_passers::CalculationStage::End,
            data: HashMap::new(),
            windows_opened: WindowsOpened::default(),
            settings: settings::Settings::default(),

            message_passers: message_passers::MessagePassers::default(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(data) = self.message_passers.calculator_to_main_receiver.try_recv() {
            match data {
                message_passers::Message::NewPoint(point) => {
                    let entry = self.data.entry(point.colour).or_default();
                    entry.push(point);
                }
                message_passers::Message::NewStage(stage) => self.calculation_stage = stage,
            }
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.selectable_value(&mut self.window_to_show, WindowToShow::LatitudeVsTimeGraph, "Graph of latitude vs time");
                    ui.selectable_value(&mut self.window_to_show, WindowToShow::LongitudeVsTimeGraph, "Graph of longitude vs time");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.calculation_stage == message_passers::CalculationStage::End {
                        if ui.button("Recalculate").clicked() {
                            self.recalculate();
                        }
                    } else {
                        ui.add_enabled(false, egui::Button::new(self.calculation_stage.as_ref()));
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
        self.data = HashMap::new();
        self.calculation_stage = message_passers::CalculationStage::Start;

        let settings = self.settings.clone();
        let sender = self.message_passers.calculator_to_main_sender.clone();
        std::thread::spawn(move || simulator::recalculate_simulation(settings, sender));
    }
}

#[derive(Default)]
pub struct WindowsOpened {
    pub settings: bool,
}
