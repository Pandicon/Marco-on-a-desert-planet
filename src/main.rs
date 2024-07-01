// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod application;
pub mod data;
pub mod rendering;
pub mod settings;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    eframe::run_native("Marco on a desert planet", Default::default(), Box::new(|cc| Box::new(application::Application::new(cc))))
}
