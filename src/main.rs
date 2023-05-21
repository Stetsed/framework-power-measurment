use csv::Writer;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a name for the run.");
        return Ok(());
    }

    let mut file_voltage = File::open("/sys/class/power_supply/BAT1/voltage_now")?;
    let mut file_current = File::open("/sys/class/power_supply/BAT1/current_now")?;

    let path = format!("{}.csv", args[1]);

    let mut wtr = Writer::from_path(path)?;

    let mut voltage = String::new();
    let mut current = String::new();

    for count in 0..600 {
        file_voltage.seek(SeekFrom::Start(0))?;
        file_current.seek(SeekFrom::Start(0))?;

        file_voltage.read_to_string(&mut voltage)?;

        let voltage_f64 = voltage.trim().parse::<f64>()? / 1_000_000.0;

        file_current.read_to_string(&mut current)?;

        let current_f64 = current.trim().parse::<f64>()? / 1_000_000.0;

        let wattage = voltage_f64 * current_f64;

        wtr.write_record(&[&count.to_string(), &args[1], &wattage.to_string()])?;
        wtr.flush()?;

        voltage = String::new();
        current = String::new();

        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}
