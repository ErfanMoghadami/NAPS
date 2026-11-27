use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Company and production location
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Company {
    pub company_id: String,
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductionSite {
    pub production_site_id: String,
    pub company_id: String,
    pub code: String,
    pub name: String,
    /// IANA timezone, for example `Asia/Baku`.
    pub timezone: String,
    pub is_active: bool,
}

// -----------------------------------------------------------------------------
// Machines and capabilities
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Machine {
    pub machine_id: String,
    pub company_id: String,
    pub production_site_id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capability {
    pub capability_id: String,
    pub company_id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

/// A time-bounded machine-to-capability qualification. The ID allows the same
/// machine/capability pair to have multiple non-overlapping validity periods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MachineCapability {
    pub machine_capability_id: String,
    pub machine_id: String,
    pub capability_id: String,
    pub proficiency_level: Option<u8>,
    /// Lower values represent a stronger preference during machine selection.
    pub priority: u8,
    pub valid_from: Option<NaiveDate>,
    pub valid_until: Option<NaiveDate>,
    pub notes: Option<String>,
}

// -----------------------------------------------------------------------------
// Products and reusable production routings
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub product_id: String,
    pub company_id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    /// Company-defined unit code, for example `pcs`, `kg`, or `meter`.
    pub base_unit_of_measure: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProductRoutingStatus {
    Draft,
    Approved,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductRouting {
    pub routing_id: String,
    pub product_id: String,
    pub name: String,
    pub version: u32,
    /// Only approved routings should be selectable for new production orders.
    pub status: ProductRoutingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductRoutingStep {
    pub routing_step_id: String,
    pub routing_id: String,
    pub sequence: u32,
    pub name: String,
    pub setup_time_seconds: i64,
    pub processing_time_per_unit_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoutingStepCapabilityRequirement {
    pub routing_step_id: String,
    pub capability_id: String,
    pub minimum_proficiency_level: Option<u8>,
}

// -----------------------------------------------------------------------------
// Production orders and their operation snapshots
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Draft,
    Released,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Order {
    pub order_id: String,
    pub company_id: String,
    pub product_id: String,
    /// The exact routing version selected when this order is created/released.
    pub routing_id: String,
    pub quantity: i64,
    pub due_at: DateTime<Utc>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Pending,
    Ready,
    Scheduled,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Operation {
    pub operation_id: String,
    pub order_id: String,
    pub routing_step_id: String,
    pub sequence: u32,
    /// If present, the APS must use this machine. If absent, the APS may choose
    /// any compatible available machine.
    pub fixed_machine_id: Option<String>,
    /// Snapshot of standard times at the time this operation is generated.
    pub setup_time_seconds: i64,
    pub processing_time_per_unit_seconds: i64,
    pub status: OperationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationCapabilityRequirement {
    pub operation_id: String,
    pub capability_id: String,
    pub minimum_proficiency_level: Option<u8>,
}

// -----------------------------------------------------------------------------
// Work calendars, shifts, and machine availability
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkCalendar {
    pub calendar_id: String,
    pub production_site_id: String,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Shift {
    pub shift_id: String,
    pub calendar_id: String,
    pub name: String,
    /// ISO-8601 weekdays: Monday = 1 through Sunday = 7.
    pub days_of_week: Vec<u8>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CalendarExceptionKind {
    /// The date is a non-working day, overriding the normal shift pattern.
    NonWorkingDay,
    /// The date is a working day, even if it is not in the normal shift pattern.
    AdditionalWorkingDay,
    /// The normal working hours are replaced by the specified time range.
    ModifiedWorkingHours,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarException {
    pub calendar_exception_id: String,
    pub calendar_id: String,
    pub date: NaiveDate,
    pub kind: CalendarExceptionKind,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MachineCalendarAssignment {
    pub assignment_id: String,
    pub machine_id: String,
    pub calendar_id: String,
    pub valid_from: NaiveDate,
    pub valid_until: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MachineDowntime {
    pub downtime_id: String,
    pub machine_id: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub reason: String,
    pub is_planned: bool,
}

// -----------------------------------------------------------------------------
// APS planning runs and resulting operation schedules
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlanningRunStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanningRun {
    pub planning_run_id: String,
    pub company_id: String,
    pub production_site_id: String,
    pub horizon_start: DateTime<Utc>,
    pub horizon_end: DateTime<Utc>,
    pub status: PlanningRunStatus,
    /// Stored so a plan can be traced to the engine version that produced it.
    pub algorithm_version: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationScheduleStatus {
    Proposed,
    Committed,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationSchedule {
    pub operation_schedule_id: String,
    pub planning_run_id: String,
    pub operation_id: String,
    pub machine_id: String,
    /// Start includes setup; end is after the operation's processing is complete.
    pub planned_start_at: DateTime<Utc>,
    pub planned_end_at: DateTime<Utc>,
    pub status: OperationScheduleStatus,
}
