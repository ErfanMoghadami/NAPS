use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Machine {
    pub machine_id: String,
    pub name: String,
    pub available_seconds_per_day: i64,
    pub is_active: bool,
}
#[derive(Debug, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub product: String,
    pub quantity: i64,
    pub due_at: String,
}
#[derive(Debug, Deserialize)]
pub struct Operation {
    pub operation_id: String,
    pub order_id: String,
    pub machine_id: String,
    pub sequence: i64,
    pub setup_time_seconds: i64,
    pub processing_time_per_unit_seconds: i64,
}
