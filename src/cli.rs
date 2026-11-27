use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long, default_value = "sample_data/companies.csv")]
    pub companies: PathBuf,

    #[arg(long, default_value = "sample_data/production_sites.csv")]
    pub production_sites: PathBuf,

    #[arg(long, default_value = "sample_data/machines.csv")]
    pub machines: PathBuf,

    #[arg(long, default_value = "sample_data/capabilities.csv")]
    pub capabilities: PathBuf,

    // Keep the existing CLI spelling for compatibility.
    #[arg(
        long = "machines-capability",
        default_value = "sample_data/machines_capability.csv"
    )]
    pub machines_capability: PathBuf,

    #[arg(long, default_value = "sample_data/products.csv")]
    pub products: PathBuf,

    #[arg(long, default_value = "sample_data/product_routings.csv")]
    pub product_routings: PathBuf,

    #[arg(long, default_value = "sample_data/product_routing_steps.csv")]
    pub product_routing_steps: PathBuf,

    #[arg(
        long,
        default_value = "sample_data/routing_step_capability_requirements.csv"
    )]
    pub routing_step_capability_requirements: PathBuf,

    #[arg(long, default_value = "sample_data/orders.csv")]
    pub orders: PathBuf,

    #[arg(long, default_value = "sample_data/operations.csv")]
    pub operations: PathBuf,

    #[arg(
        long,
        default_value = "sample_data/operation_capability_requirements.csv"
    )]
    pub operation_capability_requirements: PathBuf,

    #[arg(long, default_value = "sample_data/work_calendars.csv")]
    pub work_calendars: PathBuf,

    #[arg(long, default_value = "sample_data/shifts.csv")]
    pub shifts: PathBuf,

    #[arg(long, default_value = "sample_data/calendar_exceptions.csv")]
    pub calendar_exceptions: PathBuf,

    #[arg(
        long,
        default_value = "sample_data/machine_calendar_assignments.csv"
    )]
    pub machine_calendar_assignments: PathBuf,

    #[arg(long, default_value = "sample_data/machine_downtimes.csv")]
    pub machine_downtimes: PathBuf,
}

pub fn cli() -> CliArgs {
    CliArgs::parse()
}