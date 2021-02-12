mod account;
mod transaction;
use account::Account;
use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::{args, Args},
    io, process,
};
use transaction::TransactionType;

// io
#[derive(Debug, Deserialize)]
struct InputRecord {
    #[serde(rename = "type")]
    tx_type: String,
    client: u16,
    tx: u32,
    amount: Option<Decimal>,
}

#[derive(Debug, Serialize)]
struct OutputRecord {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

impl From<&Account> for OutputRecord {
    fn from(acct: &Account) -> Self {
        OutputRecord {
            client: acct.id(),
            available: acct.available(),
            held: acct.held(),
            total: acct.total(),
            locked: acct.locked(),
        }
    }
}

fn get_path(args: Args) -> Result<String> {
    let mut args = args;
    args.next(); // ignore the first arg, it's the executable name.

    match args.next() {
        Some(path) => Ok(path),
        None => Err(anyhow!("no path arg provided.")),
    }
}

fn read_csv(path: String) -> Result<Vec<InputRecord>> {
    let mut reader = ReaderBuilder::new().from_path(path)?;
    let mut records = vec![];

    for result in reader.deserialize() {
        let record: InputRecord = result?;
        records.push(record);
    }

    Ok(records)
}

fn write_csv(accounts: &HashMap<u16, Account>) -> Result<()> {
    let mut writer = csv::Writer::from_writer(io::stdout());

    for acct in accounts.values() {
        let record = OutputRecord::from(acct);
        writer.serialize(record)?
    }

    writer.flush()?;

    Ok(())
}

pub fn run() -> Result<()> {
    let path = match get_path(args()) {
        Ok(s) => s,
        Err(_) => {
            println!("Please supply a path to a csv file.");
            process::exit(1);
        }
    };

    let records = read_csv(path)?;
    let mut accounts: HashMap<u16, Account> = HashMap::default();

    for record in records.into_iter() {
        // for each input record, get the client.
        let client_id = record.client;
        let acct = accounts.entry(client_id).or_insert(Account::new(client_id));

        // create a transaction based on the type.
        let tx = match &record.tx_type[..] {
            "deposit" => Some(TransactionType::Deposit(record.into())),
            "withdrawal" => Some(TransactionType::Withdrawal(record.into())),
            "dispute" => Some(TransactionType::Dispute(record.into())),
            "resolve" => Some(TransactionType::Resolve(record.into())),
            "chargeback" => Some(TransactionType::Chargeback(record.into())),
            _ => None,
        };

        // apply the transaction to the client's account if it's valid.
        // note: this ignores all unknown transaction types.
        if let Err(e) = acct.apply(tx) {
            eprintln!("{}", e);
        }
    }

    write_csv(&accounts)
}
