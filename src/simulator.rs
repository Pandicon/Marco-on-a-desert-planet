use std::f64::consts::PI;
use std::sync::mpsc;

use nalgebra::Vector3;

use crate::data;
use crate::message_passers;
use crate::settings;

pub fn recalculate_simulation(settings: settings::Settings, sender: mpsc::Sender<message_passers::Message>) {
    if let Err(err) = sender.send(message_passers::Message::NewStage(message_passers::CalculationStage::Points)) {
        println!("Error sending new stage (points): {err}");
    }
    let mut data = Vec::new();
    let mut time = 0.0;
    let start_lat = settings.start_lat as f64;
    let start_lon = settings.start_lon as f64;
    let axis_tilt = PI / 2.0 - settings.rotational_axis_tilt as f64;

    let planet_rotation_axis = Vector3::new(axis_tilt.cos(), 0.0, axis_tilt.sin()).normalize();
    let planet_rotation_quaternion = nalgebra::UnitQuaternion::new(planet_rotation_axis * (2.0 * PI) / (settings.rotational_period * 3600.0) * settings.timestep * (-1.0)); // Multiplied by -1 to make the star orbit the planet in the correct direction

    let mut marco_pos = Vector3::new(start_lat.cos() * start_lon.cos(), start_lat.cos() * start_lon.sin(), start_lat.sin()) * settings.planet_radius;
    let mut sun_pos_norm = Vector3::new(1.0_f64, 0.0, 0.0);

    let point = data::Data::from_raw(marco_pos, time, settings.planet_radius);
    data.push(point.clone());
    if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
        println!("Error sending new point: {err}\nPoint: {:?}", point);
    }

    while time <= settings.simulation_time {
        let marco_to_sun = (settings.sun_distance * sun_pos_norm - marco_pos).normalize();

        // Only move Marco when the star is above his horizon
        if marco_to_sun.dot(&marco_pos) >= 0.0 {
            let rotation_axis = (marco_pos.normalize().cross(&marco_to_sun)).normalize() * (settings.marco_velocity / 1000.0 * settings.timestep) / settings.planet_radius;
            let rotation_quaternion = nalgebra::UnitQuaternion::new(rotation_axis);

            marco_pos = rotation_quaternion * marco_pos;
        }

        sun_pos_norm = planet_rotation_quaternion * sun_pos_norm;

        if time / settings.simulation_time > (data.len() as f64) / (settings.points_to_show as f64) {
            let point = data::Data::from_raw(marco_pos, time, settings.planet_radius);
            data.push(point.clone());
            if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
                println!("Error sending new point: {err}\nPoint: {:?}", point);
            }
        }

        time += settings.timestep;
    }
    let point = data::Data::from_raw(marco_pos, time, settings.planet_radius);
    data.push(point.clone());
    if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
        println!("Error sending new point: {err}\nPoint: {:?}", point);
    }
    if let Err(err) = sender.send(message_passers::Message::NewStage(message_passers::CalculationStage::End)) {
        println!("Error sending new stage (end): {err}");
    }
}
