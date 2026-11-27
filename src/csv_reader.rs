use crate::models::{
    CalendarException,
    Capability,
    Company,
    Machine,
    MachineCalendarAssignment,
    MachineCapability,
    MachineDowntime,
    Operation,
    OperationCapabilityRequirement,
    Order,
    Product,
    ProductRouting,
    ProductRoutingStep,
    ProductionSite,
    RoutingStepCapabilityRequirement,
    Shift,
    WorkCalendar,
};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

use std::path::Path;

fn read_csv<T>(path: &Path) -> Result<Vec<T>, csv::Error>
where
    T: DeserializeOwned,
{
    let mut reader = csv::Reader::from_path(path)?;

    reader.deserialize().collect()
}

pub fn companies_csv_reader(
    path: &Path,
) -> Result<Vec<Company>, csv::Error> {
    read_csv(path)
}

pub fn production_site_csv_reader(
    path: &Path,
) -> Result<Vec<ProductionSite>, csv::Error> {
    read_csv(path)
}

pub fn machine_csv_reader(
    path: &Path,
) -> Result<Vec<Machine>, csv::Error> {
    read_csv(path)
}

pub fn capability_csv_reader(
    path: &Path,
) -> Result<Vec<Capability>, csv::Error> {
    read_csv(path)
}

pub fn machine_capability_csv_reader(
    path: &Path,
) -> Result<Vec<MachineCapability>, csv::Error> {
    read_csv(path)
}

pub fn product_csv_reader(
    path: &Path,
) -> Result<Vec<Product>, csv::Error> {
    read_csv(path)
}

pub fn product_routing_csv_reader(
    path: &Path,
) -> Result<Vec<ProductRouting>, csv::Error> {
    read_csv(path)
}

pub fn product_routing_step_csv_reader(
    path: &Path,
) -> Result<Vec<ProductRoutingStep>, csv::Error> {
    read_csv(path)
}

pub fn routing_step_capability_requirement_csv_reader(
    path: &Path,
) -> Result<Vec<RoutingStepCapabilityRequirement>, csv::Error> {
    read_csv(path)
}

pub fn order_csv_reader(
    path: &Path,
) -> Result<Vec<Order>, csv::Error> {
    read_csv(path)
}

pub fn operation_csv_reader(
    path: &Path,
) -> Result<Vec<Operation>, csv::Error> {
    read_csv(path)
}

pub fn operation_capability_requirement_csv_reader(
    path: &Path,
) -> Result<Vec<OperationCapabilityRequirement>, csv::Error> {
    read_csv(path)
}

pub fn work_calendar_csv_reader(
    path: &Path,
) -> Result<Vec<WorkCalendar>, csv::Error> {
    read_csv(path)
}

// -----------------------------------------------------------------------------
// Shift CSV Reader
// -----------------------------------------------------------------------------
//
// CSV example:
//
// shift_id,calendar_id,name,days_of_week,start_time,end_time
// S001,WC001,Day Shift,1|2|3|4|5,08:00:00,16:00:00
//
// days_of_week accepts |, ; and , as separators.

#[derive(Deserialize)]
struct ShiftCsvRow {
    shift_id: String,
    calendar_id: String,
    name: String,

    #[serde(deserialize_with = "deserialize_days_of_week")]
    days_of_week: Vec<u8>,

    start_time: chrono::NaiveTime,
    end_time: chrono::NaiveTime,
}

fn deserialize_days_of_week<'de, D>(
    deserializer: D,
) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;

    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }

    raw.split(|character| {
        character == '|'
            || character == ';'
            || character == ','
    })
    .map(|part| {
        part.trim()
            .parse::<u8>()
            .map_err(serde::de::Error::custom)
    })
    .collect()
}

pub fn shift_csv_reader(
    path: &Path,
) -> Result<Vec<Shift>, csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;

    reader
        .deserialize::<ShiftCsvRow>()
        .map(|row| {
            row.map(|row| Shift {
                shift_id: row.shift_id,
                calendar_id: row.calendar_id,
                name: row.name,
                days_of_week: row.days_of_week,
                start_time: row.start_time,
                end_time: row.end_time,
            })
        })
        .collect()
}

pub fn calendar_exception_csv_reader(
    path: &Path,
) -> Result<Vec<CalendarException>, csv::Error> {
    read_csv(path)
}

pub fn machine_calendar_assignment_csv_reader(
    path: &Path,
) -> Result<Vec<MachineCalendarAssignment>, csv::Error> {
    read_csv(path)
}

pub fn machine_downtime_csv_reader(
    path: &Path,
) -> Result<Vec<MachineDowntime>, csv::Error> {
    read_csv(path)
}