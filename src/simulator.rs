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

    let mut marco_positions = vec![Vector3::new(start_lat.cos() * start_lon.cos(), start_lat.cos() * start_lon.sin(), start_lat.sin()) * settings.planet_radius; velocities_count];
    let mut sun_pos_norm = Vector3::new(axis_tilt.sin(), 0.0, axis_tilt.cos());

    let mut ecliptic_axis = sun_pos_norm.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
    let planet_rotation_axis = Vector3::new(0.0, 0.0, 1.0).normalize();
    let planet_rotation_quaternion = nalgebra::UnitQuaternion::new(planet_rotation_axis * (2.0 * PI) / (settings.rotational_period * 3600.0) * settings.timestep * (-1.0)); // Multiplied by -1 to make the star orbit the planet in the correct direction

    let red = hsluv::rgb_to_hsluv(1.0, 0.0, 0.0);
    let blue = hsluv::rgb_to_hsluv(0.0, 0.0, 1.0);
    let colours = (0..velocities_count)
        .map(|i| {
            let vel_i = i as f32;
            let rgb = hsluv::hsluv_to_rgb(red.0 + (blue.0 - red.0) * (vel_i / vels_count as f32) as f64, red.1, red.2);
            eframe::egui::Color32::from_rgba_unmultiplied((rgb.0 * 255.0) as u8, (rgb.1 * 255.0) as u8, (rgb.2 * 255.0) as u8, ((1.0 / vels_count as f32).max(1.0) * 255.0) as u8)
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

        // Everything that is not meant to be stationary with respect to the surface of the Earth has to be rotated in the opposite direction to the Earth if the surface of the Earth is to be stationary with respect to the coordinate system
        sun_pos_norm = planet_rotation_quaternion * sun_pos_norm;
        ecliptic_axis = planet_rotation_quaternion * ecliptic_axis;

        let planet_orbit_quaternion = nalgebra::UnitQuaternion::new(ecliptic_axis * (2.0 * PI) / (settings.orbital_period * 365.25 * 86400.0) * settings.timestep); // Here we should not multiply by -1 as the direction of orbit of the planet around the star and of the star around the planet are the same
        sun_pos_norm = planet_orbit_quaternion * sun_pos_norm;

        time += settings.timestep;
    }
    for i in 0..velocities_count {
        let point = data::Data::from_raw(marco_positions[i], time, settings.planet_radius, colours[i]);
        data[i].push(point.clone());
        if let Err(err) = sender.send(message_passers::Message::NewPoint(point.clone())) {
            println!("Error sending new point: {err}\nPoint: {:?}", point);
        }
    }
    if settings.generate_image {
        if let Err(err) = sender.send(message_passers::Message::NewStage(message_passers::CalculationStage::Plots)) {
            println!("Error sending new stage (plots): {err}");
        }
        if let Err(err) = generate_image(data, settings) {
            println!("Failed to generate the plot: {err}");
        }
    }
    if let Err(err) = sender.send(message_passers::Message::NewStage(message_passers::CalculationStage::End)) {
        println!("Error sending new stage (end): {err}");
    }
}

fn generate_image(data: Vec<Vec<data::Data>>, settings: settings::Settings) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

    let scale_factor = 20;
    let font = "sans";

    let planet_radius = settings.planet_radius;

    let out_file_name = "plotters-doc-data/3d-plot.png";

    let area = BitMapBackend::new(out_file_name, (1024 * scale_factor, 760 * scale_factor)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (-(planet_radius * 1.1)..(planet_radius * 1.1)).step(planet_radius * 1.1 / 100.0);
    let z_axis = (-(planet_radius * 1.1)..(planet_radius * 1.1)).step(planet_radius * 1.1 / 100.0);

    let mut chart =
        ChartBuilder::on(&area)
            .caption("Marco on a desert planet", (font, 20 * scale_factor))
            .build_cartesian_3d(x_axis.clone(), -(planet_radius * 1.1)..(planet_radius * 1.1), z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5 + std::f64::consts::PI;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(ShapeStyle::from(BLACK.mix(0.15)).stroke_width(scale_factor))
        .max_light_lines(3)
        .label_style((font, 10 * scale_factor))
        .draw()?;

    {
        // Draw the planet
        let resolution_lines_of_longitude = 10_000; // planet_radius as i32;
        let lines_of_longitude_count = 2000;
        let t = (0..=resolution_lines_of_longitude).map(|t| t as f64 / (resolution_lines_of_longitude as f64) * std::f64::consts::TAU);
        let y = t.clone().map(|t| planet_radius * t.sin());
        let z = t.map(|t| planet_radius * t.cos());
        for i in 0..lines_of_longitude_count {
            let lon = i as f64 / (lines_of_longitude_count as f64) * std::f64::consts::PI;
            let x = z.clone().map(|z| z * lon.sin());
            let z = z.clone().map(|z| z * lon.cos());
            let iter = x.zip(y.clone()).zip(z).map(|((x, y), z)| (x, y, z));
            chart.draw_series(LineSeries::new(iter, ShapeStyle::from(BLACK.mix(0.3)).stroke_width((scale_factor / 4).max(1))))?;
        }
    }

    // Draw the paths
    for i in (0..data.len()).rev() {
        if data[i].is_empty() {
            continue;
        }
        chart.draw_series(LineSeries::new(
            (0..data[i].len()).map(|t| {
                let point = &data[i][t];
                let lat = point.latitude.to_radians();
                let lon = point.longitude.to_radians();
                let (x, y, z) = (planet_radius * lat.cos() * lon.cos(), planet_radius * lat.cos() * lon.sin(), planet_radius * lat.sin());
                (y, z, x)
            }),
            ShapeStyle::from(RGBAColor(data[i][0].colour.r(), data[i][0].colour.g(), data[i][0].colour.b(), data[i][0].colour.a() as f64 / 255.0)),
        ))?;
    }

    /*chart
        .draw_series(
            SurfaceSeries::xoz(
                (-100..100).map(|f| f as f64 / 100.0 * (radius * 1.1)),
                (-100..100).map(|f| f as f64 / 100.0 * (radius * 1.1)),
                |x, z| {
                    let y_2 = radius * radius - (x * x + z * z);
                    if y_2 < 0.0 {
                        0.0
                    } else {
                        y_2.sqrt()
                    }
                },
            )
            .style(BLUE.mix(0.2).filled()),
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-100..100).map(|f| f as f64 / 100.0 * (radius * 1.1)),
                (-100..100).map(|f| f as f64 / 100.0 * (radius * 1.1)),
                |x, z| {
                    let y_2 = radius * radius - (x * x + z * z);
                    if y_2 < 0.0 {
                        0.0
                    } else {
                        -y_2.sqrt()
                    }
                },
            )
            .style(BLUE.mix(0.2).filled()),
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));*/

    /*chart
    .draw_series(LineSeries::new(
        (-100..100)
            .map(|y| y as f64 / 100.0 * (radius * 1.1))
            .map(|y| ((radius * 1.1) * (y * 10.0).sin(), y, (radius * 1.1) * (y * 10.0).cos())),
        &BLACK,
    ))?
    .label("Line")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));*/

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present()
        .expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", out_file_name);
    Ok(())
}
