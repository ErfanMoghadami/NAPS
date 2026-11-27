use crate::business_rules::machine_business_rules;
use crate::cli::CliArgs;

use crate::csv_reader::{
    calendar_exception_csv_reader,
    capability_csv_reader,
    companies_csv_reader,
    machine_calendar_assignment_csv_reader,
    machine_capability_csv_reader,
    machine_csv_reader,
    machine_downtime_csv_reader,
    operation_capability_requirement_csv_reader,
    operation_csv_reader,
    order_csv_reader,
    product_csv_reader,
    product_routing_csv_reader,
    product_routing_step_csv_reader,
    production_site_csv_reader,
    routing_step_capability_requirement_csv_reader,
    shift_csv_reader,
    work_calendar_csv_reader,
};

use crate::data_validation::{
    data_validation_calendar_exceptions,
    data_validation_capabilities,
    data_validation_companies,
    data_validation_machine,
    data_validation_machine_calendar_assignments,
    data_validation_machine_capabilities,
    data_validation_machine_downtimes,
    data_validation_operation_capability_requirements,
    data_validation_operations,
    data_validation_orders,
    data_validation_product_routing_steps,
    data_validation_product_routings,
    data_validation_products,
    data_validation_production_sites,
    data_validation_routing_step_capability_requirements,
    data_validation_shifts,
    data_validation_work_calendars,
    validation_reference,
};

use crate::input_files::input_validation_all_files;

use std::path::Path;

// -----------------------------------------------------------------------------
// Generic CSV loading + Data Validation
// -----------------------------------------------------------------------------

fn load_and_validate<T>(
    dataset_name: &str,
    path: &Path,
    reader: fn(&Path) -> Result<Vec<T>, csv::Error>,
    validator: fn(&[T]) -> Vec<String>,
) -> Option<Vec<T>> {
    match reader(path) {
        Ok(data) => {
            let errors = validator(&data);

            if errors.is_empty() {
                println!("{dataset_name} data is valid.");
                Some(data)
            } else {
                for error in errors {
                    eprintln!("{dataset_name}: {error}");
                }

                None
            }
        }

        Err(error) => {
            eprintln!("Failed to parse {dataset_name}: {error}");
            None
        }
    }
}

// -----------------------------------------------------------------------------
// Main application
// -----------------------------------------------------------------------------

pub fn app(args: &CliArgs) {

    // -------------------------------------------------------------------------
    // Stage 1: Input File Validation
    // -------------------------------------------------------------------------

    let input_errors = input_validation_all_files(args);

    if !input_errors.is_empty() {
        eprintln!("Input file validation failed:");

        for error in input_errors {
            eprintln!("- {error}");
        }

        return;
    }

    println!("Input file validation passed.");

    // -------------------------------------------------------------------------
    // Stage 2: CSV Deserialization + Data Validation
    // -------------------------------------------------------------------------

    let companies_data = load_and_validate(
        "companies.csv",
        &args.companies,
        companies_csv_reader,
        data_validation_companies,
    );

    let production_sites_data = load_and_validate(
        "production_sites.csv",
        &args.production_sites,
        production_site_csv_reader,
        data_validation_production_sites,
    );

    let machines_data = load_and_validate(
        "machines.csv",
        &args.machines,
        machine_csv_reader,
        data_validation_machine,
    );

    let capabilities_data = load_and_validate(
        "capabilities.csv",
        &args.capabilities,
        capability_csv_reader,
        data_validation_capabilities,
    );

    let machine_capabilities_data = load_and_validate(
        "machines_capability.csv",
        &args.machines_capability,
        machine_capability_csv_reader,
        data_validation_machine_capabilities,
    );

    let products_data = load_and_validate(
        "products.csv",
        &args.products,
        product_csv_reader,
        data_validation_products,
    );

    let product_routings_data = load_and_validate(
        "product_routings.csv",
        &args.product_routings,
        product_routing_csv_reader,
        data_validation_product_routings,
    );

    let product_routing_steps_data = load_and_validate(
        "product_routing_steps.csv",
        &args.product_routing_steps,
        product_routing_step_csv_reader,
        data_validation_product_routing_steps,
    );

    let routing_step_capability_requirements_data = load_and_validate(
        "routing_step_capability_requirements.csv",
        &args.routing_step_capability_requirements,
        routing_step_capability_requirement_csv_reader,
        data_validation_routing_step_capability_requirements,
    );

    let orders_data = load_and_validate(
        "orders.csv",
        &args.orders,
        order_csv_reader,
        data_validation_orders,
    );

    let operations_data = load_and_validate(
        "operations.csv",
        &args.operations,
        operation_csv_reader,
        data_validation_operations,
    );

    let operation_capability_requirements_data = load_and_validate(
        "operation_capability_requirements.csv",
        &args.operation_capability_requirements,
        operation_capability_requirement_csv_reader,
        data_validation_operation_capability_requirements,
    );

    let work_calendars_data = load_and_validate(
        "work_calendars.csv",
        &args.work_calendars,
        work_calendar_csv_reader,
        data_validation_work_calendars,
    );

    let shifts_data = load_and_validate(
        "shifts.csv",
        &args.shifts,
        shift_csv_reader,
        data_validation_shifts,
    );

    let calendar_exceptions_data = load_and_validate(
        "calendar_exceptions.csv",
        &args.calendar_exceptions,
        calendar_exception_csv_reader,
        data_validation_calendar_exceptions,
    );

    let machine_calendar_assignments_data = load_and_validate(
        "machine_calendar_assignments.csv",
        &args.machine_calendar_assignments,
        machine_calendar_assignment_csv_reader,
        data_validation_machine_calendar_assignments,
    );

    let machine_downtimes_data = load_and_validate(
        "machine_downtimes.csv",
        &args.machine_downtimes,
        machine_downtime_csv_reader,
        data_validation_machine_downtimes,
    );

    // -------------------------------------------------------------------------
    // Stage 2.1: Stop if any dataset failed to load or validate
    // -------------------------------------------------------------------------

    let (
        Some(companies),
        Some(production_sites),
        Some(machines),
        Some(capabilities),
        Some(machine_capabilities),
        Some(products),
        Some(product_routings),
        Some(product_routing_steps),
        Some(routing_step_capability_requirements),
        Some(orders),
        Some(operations),
        Some(operation_capability_requirements),
        Some(work_calendars),
        Some(shifts),
        Some(calendar_exceptions),
        Some(machine_calendar_assignments),
        Some(machine_downtimes),
    ) = (
        companies_data,
        production_sites_data,
        machines_data,
        capabilities_data,
        machine_capabilities_data,
        products_data,
        product_routings_data,
        product_routing_steps_data,
        routing_step_capability_requirements_data,
        orders_data,
        operations_data,
        operation_capability_requirements_data,
        work_calendars_data,
        shifts_data,
        calendar_exceptions_data,
        machine_calendar_assignments_data,
        machine_downtimes_data,
    )
    else {
        eprintln!(
            "Data validation failed: one or more datasets are missing or invalid."
        );

        return;
    };

    // -------------------------------------------------------------------------
    // Stage 3: Reference Validation
    // -------------------------------------------------------------------------

    let reference_errors = validation_reference(
        &companies,
        &production_sites,
        &machines,
        &capabilities,
        &machine_capabilities,
        &products,
        &product_routings,
        &product_routing_steps,
        &routing_step_capability_requirements,
        &orders,
        &operations,
        &operation_capability_requirements,
        &work_calendars,
        &shifts,
        &calendar_exceptions,
        &machine_calendar_assignments,
        &machine_downtimes,
    );

    if !reference_errors.is_empty() {
        eprintln!("Reference validation failed:");

        for error in reference_errors {
            eprintln!("- {error}");
        }

        return;
    }

    println!("Reference validation passed.");

    // -------------------------------------------------------------------------
    // Stage 4: Business Rules
    // -------------------------------------------------------------------------

    let business_errors =
        machine_business_rules(&machines, &operations);

    if business_errors.is_empty() {
        println!("Current machine business rules passed.");
    } else {
        eprintln!("Business rules found issues:");

        for error in business_errors {
            eprintln!("- {error}");
        }
    }
}