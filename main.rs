extern crate csv;
extern crate serde;

use serde::{Deserialize, Serialize}; // Import Serialize
use std::error::Error;
use std::fs::File;
use std::process;

#[derive(Debug, Deserialize, Serialize)] // Derive Serialize here
struct Record {
    name: String,
    age: u32,
}

fn process_csv(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    // Open the input file
    let file = File::open(input_file)?;

    // Create a CSV reader
    let mut rdr = csv::Reader::from_reader(file);

    // Open the output file
    let file = File::create(output_file)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Iterate over each record
    for result in rdr.deserialize() {
        // Deserialize each record
        let record: Record = result?;
        
        // Simple processing: filter records where age is over 30
        if record.age > 30 {
            // Write the record to the output file
            wtr.serialize(&record)?;
        }
    }

    // Flush writes to the output file
    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Err(err) = process_csv("input.csv", "output.csv") {
        println!("Error processing CSV file: {}", err);
        process::exit(1);
    }
}
