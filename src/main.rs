use anyhow::Result;
use banky;
// 1. model clients. a struct.
// 1. model transactions. maybe a trait object
// 1. read the csv file and turn the data into transactions.
fn main() -> Result<()> {
    banky::run()
}
