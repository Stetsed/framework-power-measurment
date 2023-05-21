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

    let mut wtr = Writer::from_path("output.csv")?;

    let mut voltage = String::new();
    let mut current = String::new();

    for count in 0..600 {
        file_voltage.seek(SeekFrom::Start(0))?;
        file_current.seek(SeekFrom::Start(0))?;

        file_voltage.read_to_string(&mut voltage)?;

        let voltage = voltage.trim().parse::<f64>()? / 1_000_000.0;

        file_current.read_to_string(&mut current)?;

        let current = current.trim().parse::<f64>()? / 1_000_000.0;

        let wattage = voltage * current;

        wtr.write_record(&[&count.to_string(), &args[1], &wattage.to_string()])?;
        wtr.flush()?;

        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}
