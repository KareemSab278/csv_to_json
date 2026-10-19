/*
    to get this project started run:
            cargo runcargo add anyhow@1.0.104
            cargo add csv@1.4.0
            cargo add serde@1.0.229 --features derive
            cargo add serde_json@1.0.154
            cargo add spreadsheet-to-json@0.5.0
    
    then create the csv file with python3 create_csv.py

    run 'cargo check' for any errors; 
    'cargo run' to test the project
*/

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub product_id: i32,
    pub product_name: String,
    pub product_category: String,
    pub product_price: f64,
    pub product_availability: bool,
}

pub fn csv_to_json<T, R>(reader: R) -> Result<String>
where
    T: DeserializeOwned + Serialize,
    R: Read,
{
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut items: Vec<T> = Vec::new();

    for (index, row) in csv_reader.deserialize::<T>().enumerate() {
        let row_number = index + 2; // Header is row 1.

        let item: T = row.with_context(|| {
            format!(
                "Failed to map CSV row {} to the requested struct. \
                 Check column names, required fields, and data types.",
                row_number
            )
        })?;

        items.push(item);
    }

    serde_json::to_string_pretty(&items)
        .context("Failed to serialize the validated records to JSON")
}