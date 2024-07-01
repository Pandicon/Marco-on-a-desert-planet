use std::f64::consts::PI;
use std::sync::mpsc;

use nalgebra::Vector3;

use crate::data;
use crate::message_passers;
use crate::settings;

pub fn recalculate_simulation(settings: settings::Settings, sender: mpsc::Sender<message_passers::Message>) {
    let velocities_count = settings.velocities_count.max(1);
    let vels_count = velocities_count as f64;

    if let Err(err) = sender.send(message_passers::Message::NewStage(message_passers::CalculationStage::Points)) {
        println!("Error sending new stage (points): {err}");
    }
    let mut data = vec![Vec::new(); velocities_count];
    let mut time = 0.0;
    let start_lat = settings.start_lat as f64;
    let start_lon = settings.start_lon as f64;
    let axis_tilt = PI / 2.0 - settings.rotational_axis_tilt as f64;

    let planet_rotation_axis = Vector3::new(axis_tilt.cos(), 0.0, axis_tilt.sin()).normalize();
    let planet_rotation_quaternion = nalgebra::UnitQuaternion::new(planet_rotation_axis * (2.0 * PI) / (settings.rotational_period * 3600.0) * settings.timestep * (-1.0)); // Multiplied by -1 to make the star orbit the planet in the correct direction

    let mut marco_positions = vec![Vector3::new(start_lat.cos() * start_lon.cos(), start_lat.cos() * start_lon.sin(), start_lat.sin()) * settings.planet_radius; velocities_count];
    let mut sun_pos_norm = Vector3::new(1.0_f64, 0.0, 0.0);
    let colours = (0..velocities_count)
        .map(|i| {
            let vel_i = i as f32;
            eframe::egui::Color32::from(eframe::egui::ecolor::Hsva::new(vel_i / vels_count as f32, 1.0, 1.0, (1.0 / vels_count as f32).max(0.1)))
        })
        .collect::<Vec<eframe::egui::Color32>>();

    for i in 0..velocities_count {
        let point = data::Data::from_raw(marco_positions[i], time, settings.planet_radius, colours[i]);
        data[i].push(point.clone());
        if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
            println!("Error sending new point: {err}\nPoint: {:?}", point);
        }
    }

    while time <= settings.simulation_time {
        for i in 0..velocities_count {
            let marco_vel = settings.marco_min_velocity + (settings.marco_max_velocity - settings.marco_min_velocity) * ((i as f64) / vels_count);
            let marco_to_sun = (settings.sun_distance * sun_pos_norm - marco_positions[i]).normalize();

            // Only move Marco when the star is above his horizon
            if marco_to_sun.dot(&marco_positions[i]) >= 0.0 {
                let rotation_axis = (marco_positions[i].normalize().cross(&marco_to_sun)).normalize() * (marco_vel / 1000.0 * settings.timestep) / settings.planet_radius;
                let rotation_quaternion = nalgebra::UnitQuaternion::new(rotation_axis);

                marco_positions[i] = rotation_quaternion * marco_positions[i];
            }

            if time / settings.simulation_time > (data[i].len() as f64) / (settings.points_to_show as f64) {
                let point = data::Data::from_raw(marco_positions[i], time, settings.planet_radius, colours[i]);
                data[i].push(point.clone());
                if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
                    println!("Error sending new point: {err}\nPoint: {:?}", point);
                }
            }
        }

        sun_pos_norm = planet_rotation_quaternion * sun_pos_norm;

        time += settings.timestep;
    }
    for i in 0..velocities_count {
        let point = data::Data::from_raw(marco_positions[i], time, settings.planet_radius, colours[i]);
        data[i].push(point.clone());
        if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
            println!("Error sending new point: {err}\nPoint: {:?}", point);
        }
    }
    if let Err(err) = sender.send(message_passers::Message::NewStage(message_passers::CalculationStage::End)) {
        println!("Error sending new stage (end): {err}");
    }
}
