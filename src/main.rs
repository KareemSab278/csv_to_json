mod csv_to_json;
use csv_to_json::*;
use anyhow::{Context, Result};
use std::fs::File;

fn main() -> Result<()> {
    let file = File::open("products.csv")
        .context("Could not open products.csv")?;

    let json = csv_to_json::<Product, _>(file)
        .context("CSV-to-JSON conversion failed")?;

    println!("{}", json);

    Ok(())
}
