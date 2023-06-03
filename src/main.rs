use crossterm::{execute, terminal::EnterAlternateScreen};
use csv::Writer;
use plotters::prelude::*;
use plotters::style::{BLACK, BLUE, WHITE};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::fs::{self, DirBuilder};
use std::io::{stdout, Result, Write};
use std::io::{Read, Seek, SeekFrom};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if &args.len() < &6 {
        help()?;
    };

    if &args[1].as_str() == &"help" {
        help()?;
    };

    // Get current time and the time that's X seconds from now with X being argv[3]
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let time_until = now
        + args[3]
            .trim()
            .parse::<u64>()
            .expect("Couldn't parse time to u64");

    let args2 = args.clone();

    match args[1].as_str() {
        "stress" => tokio::spawn(async move { stress_thread(&args2, &time_until) }),
        "terminal" => tokio::spawn(async move { terminal_spam(&time_until) }),
        _ => tokio::spawn(async move { Ok(()) }),
    };

    measure(args.clone(), &time_until).await;

    Ok(())
}

fn help() -> Result<()> {
    println!("Usage:");
    println!("  <program> help                   - Show this help section.");
    println!("  <program> stress <threads> <time> <output> <info>   - Perform stress operation for the given amount of threads for the specified time.");
    println!("  <program> terminal - <time> <output> <info>      - Spam terminal with random characters for the specified time.");
    println!("  <program> measure - <time> <output> <info>    - Just measure the power usage ");
    std::process::exit(0);
}

fn stress_thread(args: &Vec<String>, time_until: &u64) -> Result<()> {
    let threads = &args[2];

    let thread_amount = threads
        .trim()
        .parse::<i64>()
        .expect("Couldn't parse threads to i64");
    while time_until
        > &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get time for Stress Threads")
            .as_secs()
    {
        let mut handles = vec![];
        for _ in 0..thread_amount {
            let handle = thread::spawn(|| {
                let n: usize = 100000000;
                let mut sieve = vec![true; n + 1];
                sieve[0] = false;
                sieve[1] = false;

                for i in 2..=(n as f64).sqrt() as usize {
                    if sieve[i] {
                        let mut j = i * i;
                        while j <= n {
                            sieve[j] = false;
                            j += i;
                        }
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    Ok(())
}
fn terminal_spam(time_until: &u64) -> Result<()> {
    let mut rng = rand::thread_rng();

    // Enter alternate screen buffer
    execute!(stdout(), EnterAlternateScreen)?;

    // Loop indefinitely
    while time_until
        > &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get time for Stress Threads")
            .as_secs()
    {
        // Clear the screen
        execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;

        // Get the size of the terminal
        let (width, height) = crossterm::terminal::size()?;

        // Generate random characters and write them to the terminal
        for y in 0..height {
            for x in 0..width {
                let random_char: char = rng.sample(Alphanumeric) as char;
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x, y),
                    crossterm::style::Print(random_char)
                )?;
            }
        }

        // Flush the output
        stdout().flush()?;

        // Sleep for a short duration to control the speed
        thread::sleep(Duration::from_millis(100));
    }

    crossterm::terminal::Clear(crossterm::terminal::ClearType::All);

    Ok(())
}

async fn measure(args: Vec<String>, time_until: &u64) -> anyhow::Result<()> {
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

    let path = format!("{}{}.csv", "./measure/", &args[4]);

    let mut wtr = Writer::from_path(&path)?;

    wtr.write_record(&["Time", "Settings", "Info", "Wattage"])?;

    let mut voltage = String::new();
    let mut current = String::new();

    let settings = format!("{}-{}", &args[1], &args[2]);

    let mut count = 0;
    while time_until
        > &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get time for Stress Threads")
            .as_secs()
    {
        file_voltage.seek(SeekFrom::Start(0))?;
        file_current.seek(SeekFrom::Start(0))?;

        file_voltage.read_to_string(&mut voltage)?;

        let voltage_f64 = voltage.trim().parse::<f64>().expect("Couldn't parse f64") / 1_000_000.0;

        file_current.read_to_string(&mut current)?;

        let current_f64 = current.trim().parse::<f64>().expect("Couldn't parse f64") / 1_000_000.0;

        let wattage = voltage_f64 * current_f64;

        wtr.write_record(&[
            &count.to_string(),
            &settings,
            &args[5],
            &wattage.to_string(),
        ])?;
        wtr.flush()?;

        voltage = String::new();
        current = String::new();

        count += 1;

        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
