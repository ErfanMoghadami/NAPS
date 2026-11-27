use crate::models::{Company, Machine, MachineCapability, Operation, Order};
use std::path::Path;

pub fn machine_csv_reader(path: &Path) -> Result<Vec<Machine>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;

    reader.deserialize().collect()
}

pub fn order_csv_reader(path: &Path) -> Result<Vec<Order>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader.deserialize().collect()
}
pub fn operation_csv_reader(path: &Path) -> Result<Vec<Operation>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader.deserialize().collect()
}

pub fn machine_capability_csv_reader(path: &Path) -> Result<Vec<MachineCapability>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader.deserialize().collect()
}

pub fn companies_csv_reader(path: &Path) -> Result<Vec<Company>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;
    reader.deserialize().collect()
}
