#![allow(non_snake_case)]

use std::sync::mpsc;
use std::thread;

use crate::recipe::StatsLoader;
extern crate lazy_static;
use self::recipe::calculator::{CustomPostcodeDeliveryTime, StatsCalculator};
use chrono::Local;
use std::time::Instant;

mod recipe;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let startTime = Local::now().format("%H:%M:%S");
    let (tx, rx) = mpsc::channel();

    thread::spawn(|| {
        StatsLoader::load("./data/recipes_small.json", tx).unwrap();
    });

    let statsCalculator = StatsCalculator {
        customPostcodeDeliveryTime: CustomPostcodeDeliveryTime {
            postcode: "10120".to_string(),
            from: 10,
            to: 3,
        },
        customRecipeNames: Vec::from([
            "Potato".to_string(),
            "Veggie".to_string(),
            "Mushroom".to_string(),
        ]),
    };

    let jsonExpectedOutput: String =
        serde_json::to_string_pretty(&statsCalculator.calculateStats(rx))?;

    println!("{}", jsonExpectedOutput);

    println!("\nStarted At: {}", startTime);
    println!("Finished At: {}", Local::now().format("%H:%M:%S"));

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
