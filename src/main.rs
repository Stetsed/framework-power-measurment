use csv::Writer;
use plotters::prelude::*;
use plotters::style::IntoFont;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide a name for the run, and the task in the format ./(program) (settings) (task).");
        return Ok(());
    }

    let battery_dir = "/sys/class/power_supply";

    let mut battery: String = String::new();

    match fs::read_dir(battery_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let batterys = file_name.to_string_lossy().to_string();
                    if batterys.contains("BAT") {
                        battery = batterys;
                        break;
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to read directory {}: {}", battery_dir, err);
        }
    }

    let mut file_voltage = File::open(format!("/sys/class/power_supply/{}/voltage_now", &battery))?;
    let mut file_current = File::open(format!("/sys/class/power_supply/{}/current_now", &battery))?;

    let path = format!("{}-{}.csv", &args[1], &args[2]);

    let mut wtr = Writer::from_path(&path)?;

    wtr.write_record(&["Time", "Settings", "Task", "Wattage"])?;

    let mut voltage = String::new();
    let mut current = String::new();

    for count in 0..10 {
        file_voltage.seek(SeekFrom::Start(0))?;
        file_current.seek(SeekFrom::Start(0))?;

        file_voltage.read_to_string(&mut voltage)?;

        let voltage_f64 = voltage.trim().parse::<f64>()? / 1_000_000.0;

        file_current.read_to_string(&mut current)?;

        let current_f64 = current.trim().parse::<f64>()? / 1_000_000.0;

        let wattage = voltage_f64 * current_f64;

        wtr.write_record(&[&count.to_string(), &args[1], &args[2], &wattage.to_string()])?;
        wtr.flush()?;

        voltage = String::new();
        current = String::new();

        thread::sleep(time::Duration::from_secs(1));
    }

    // Parse the CSV data
    let mut rdr = csv::Reader::from_path(&path)?;

    let mut data: Vec<(f64, f64)> = Vec::new();
    for result in rdr.deserialize() {
        let record: (f64, String, String, f64) = result?;
        data.push((record.0, record.3));
    }

    let chart_path: String = format!("{}-{}.png", &args[1], &args[2]);

    // Setup chart
    let root = plotters::prelude::BitMapBackend::new(&chart_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_wattage = data
        .iter()
        .map(|(_, wattage)| wattage)
        .cloned()
        .fold(f64::NAN, f64::max);
    let max_time = data
        .iter()
        .map(|(time, _)| time)
        .cloned()
        .fold(f64::NAN, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Wattage over time", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..max_time, 0f64..max_wattage)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(data, &BLUE))?
        .label("Wattage")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
