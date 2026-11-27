use crate::cli::CliArgs;
use std::path::Path;

pub fn input_validation_one_file(
    name: &str,
    pathname: &Path,
) -> Option<String> {
    if !pathname.exists() {
        return Some(format!(
            "{name} does not exist: {}",
            pathname.display()
        ));
    }

    if !pathname.is_file() {
        return Some(format!(
            "{name} is not a file: {}",
            pathname.display()
        ));
    }

    None
}

pub fn input_validation_all_files(
    args: &CliArgs,
) -> Vec<String> {
    let mut errors = Vec::new();

    let files = [
        ("companies.csv", args.companies.as_path()),
        ("production_sites.csv", args.production_sites.as_path()),
        ("machines.csv", args.machines.as_path()),
        ("capabilities.csv", args.capabilities.as_path()),
        ("machines_capability.csv", args.machines_capability.as_path()),
        ("products.csv", args.products.as_path()),
        ("product_routings.csv", args.product_routings.as_path()),
        ("product_routing_steps.csv", args.product_routing_steps.as_path()),
        (
            "routing_step_capability_requirements.csv",
            args.routing_step_capability_requirements.as_path(),
        ),
        ("orders.csv", args.orders.as_path()),
        ("operations.csv", args.operations.as_path()),
        (
            "operation_capability_requirements.csv",
            args.operation_capability_requirements.as_path(),
        ),
        ("work_calendars.csv", args.work_calendars.as_path()),
        ("shifts.csv", args.shifts.as_path()),
        ("calendar_exceptions.csv", args.calendar_exceptions.as_path()),
        (
            "machine_calendar_assignments.csv",
            args.machine_calendar_assignments.as_path(),
        ),
        ("machine_downtimes.csv", args.machine_downtimes.as_path()),
    ];

    for (name, path) in files {
        if let Some(error) = input_validation_one_file(name, path) {
            errors.push(error);
        }
    }

    errors
}