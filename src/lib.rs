use anyhow::{anyhow, Result};
use csv::{ReaderBuilder, StringRecord};
use serde::Deserialize;
use std::error::Error;
use std::{
    env::{args, Args},
    process,
};
pub struct Account {
    id: u16,
    total: f64,
    available: f64,
    held: f64,
    locked: bool,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Account {
            id,
            total: 0.into(),
            available: 0.into(),
            held: 0.into(),
            locked: false,
        }
    }
}
// maybe account should do the applying.
// maybe the transaction can have an apply prop that holds a lambda...

// pub trait Transaction {
//     fn apply(account: &Account) -> Result<(), Box<dyn Error>>;
// }

// io
#[derive(Debug, Deserialize)]
struct InputRecord {
    r#type: String,
    client: u16,
    tx: u32,
    amount: Option<f32>,
}

fn get_path(args: Args) -> Result<String> {
    let mut args = args;
    args.next(); // ignore the first arg, it's the executable name.

    match args.next() {
        Some(path) => Ok(path),
        None => Err(anyhow!("no path arg provided.")),
    }
}

fn read_csv(path: String) -> Result<Vec<InputRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(path)?;
    let mut records = vec![];

    for result in reader.deserialize() {
        let record: InputRecord = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn run() -> Result<()> {
    let path = match get_path(args()) {
        Ok(s) => s,
        Err(_) => {
            println!("Please supply a path to a csv file.");
            process::exit(1);
        }
    };

    let records = read_csv(path);
    println!("done parsing.");
    println!("records: {:?}", records);

    Ok(())
}
